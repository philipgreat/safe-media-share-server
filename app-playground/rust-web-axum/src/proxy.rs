
//! Client IP resolution under reverse proxies (X-Forwarded-For / X-Real-IP).

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// One proxy trust rule loaded from the domain model.
#[derive(Debug, Clone)]
pub struct ProxyTrustConfig {
    /// Header to consult when the peer is a trusted proxy (e.g. `X-Forwarded-For`).
    pub header_name: String,
    /// Comma-separated CIDR list of trusted proxy addresses.
    pub trusted_proxy_cidrs: String,
    pub enabled: bool,
}

/// Parse a single CIDR (`10.0.0.0/8`, `2001:db8::/32`) or bare IP.
pub fn ip_in_cidr(ip: IpAddr, cidr: &str) -> bool {
    let cidr = cidr.trim();
    if cidr.is_empty() {
        return false;
    }
    if let Some((net_s, prefix_s)) = cidr.split_once('/') {
        let Ok(prefix) = prefix_s.parse::<u8>() else {
            return false;
        };
        match (ip, net_s.parse::<IpAddr>()) {
            (IpAddr::V4(a), Ok(IpAddr::V4(b))) => prefix <= 32 && ipv4_in_prefix(a, b, prefix),
            (IpAddr::V6(a), Ok(IpAddr::V6(b))) => prefix <= 128 && ipv6_in_prefix(a, b, prefix),
            _ => false,
        }
    } else {
        cidr.parse::<IpAddr>().ok() == Some(ip)
    }
}

fn ipv4_in_prefix(ip: Ipv4Addr, net: Ipv4Addr, prefix: u8) -> bool {
    if prefix == 0 {
        return true;
    }
    let mask = u32::MAX << (32 - prefix);
    (u32::from(ip) & mask) == (u32::from(net) & mask)
}

fn ipv6_in_prefix(ip: Ipv6Addr, net: Ipv6Addr, prefix: u8) -> bool {
    if prefix == 0 {
        return true;
    }
    let ip_u = u128::from(ip);
    let net_u = u128::from(net);
    let mask = u128::MAX << (128 - prefix);
    (ip_u & mask) == (net_u & mask)
}

/// True when `ip` appears in any comma-separated CIDR entry.
pub fn ip_in_cidr_list(ip: IpAddr, list: &str) -> bool {
    list.split(',').any(|cidr| ip_in_cidr(ip, cidr))
}

/// Normalize a forwarded client candidate: strip port and brackets.
pub fn parse_ip_candidate(raw: &str) -> Option<IpAddr> {
    let raw = raw.trim().trim_matches('"');
    if raw.is_empty() {
        return None;
    }
    if let Ok(addr) = raw.parse::<SocketAddr>() {
        return Some(addr.ip());
    }
    if let Ok(ip) = raw.parse::<IpAddr>() {
        return Some(ip);
    }
    // `[2001:db8::1]:443` or bare IPv6 with zone
    if let Some(inner) = raw.strip_prefix('[').and_then(|s| s.split(']').next()) {
        return inner.parse().ok();
    }
    None
}

/// Resolve the true client IP for a request.
///
/// `peer` is the direct TCP peer. When the peer is a trusted proxy, the
/// configured header is consulted. Within `X-Forwarded-For`, we walk from
/// right to left and return the first address that is **not** a trusted proxy
/// (classic reverse-proxy client reconstruction). Direct (non-proxy) peers are
/// returned as-is and headers are ignored — otherwise clients could spoof IPs.
pub fn resolve_client_ip(
    peer: SocketAddr,
    headers: &http::HeaderMap,
    rules: &[ProxyTrustConfig],
) -> IpAddr {
    let peer_ip = peer.ip();
    let enabled: Vec<&ProxyTrustConfig> = rules.iter().filter(|r| r.enabled).collect();
    if enabled.is_empty() {
        return peer_ip;
    }

    let peer_is_trusted = enabled
        .iter()
        .any(|r| ip_in_cidr_list(peer_ip, &r.trusted_proxy_cidrs));
    if !peer_is_trusted {
        return peer_ip;
    }

    // Prefer the first enabled rule whose header is present.
    for rule in &enabled {
        let header_name = rule.header_name.trim();
        let Some(value) = headers.get(header_name) else {
            continue;
        };
        let Ok(value) = value.to_str() else {
            continue;
        };

        // X-Real-IP is a single value; X-Forwarded-For is a list.
        if header_name.eq_ignore_ascii_case("x-real-ip") {
            if let Some(ip) = parse_ip_candidate(value) {
                return ip;
            }
            continue;
        }

        // Walk right-to-left; skip trusted proxies; first untrusted wins.
        let mut fallback = None;
        for part in value.split(',').rev() {
            let Some(ip) = parse_ip_candidate(part) else {
                continue;
            };
            if ip_in_cidr_list(ip, &rule.trusted_proxy_cidrs) {
                continue;
            }
            return ip;
        }
        // Entire chain is trusted (or empty): use the leftmost valid entry.
        if fallback.is_none() {
            for part in value.split(',') {
                if let Some(ip) = parse_ip_candidate(part) {
                    fallback = Some(ip);
                    break;
                }
            }
        }
        if let Some(ip) = fallback {
            return ip;
        }
    }
    peer_ip
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::HeaderValue;

    fn rules() -> Vec<ProxyTrustConfig> {
        vec![ProxyTrustConfig {
            header_name: "X-Forwarded-For".to_owned(),
            trusted_proxy_cidrs: "10.0.0.0/8,127.0.0.1".to_owned(),
            enabled: true,
        }]
    }

    #[test]
    fn cidr_match_v4() {
        assert!(ip_in_cidr("10.1.2.3".parse().unwrap(), "10.0.0.0/8"));
        assert!(!ip_in_cidr("192.168.1.1".parse().unwrap(), "10.0.0.0/8"));
    }

    #[test]
    fn direct_peer_ignores_forwarded_headers() {
        let peer: SocketAddr = "203.0.113.9:5555".parse().unwrap();
        let mut headers = http::HeaderMap::new();
        headers.insert("X-Forwarded-For", HeaderValue::from_static("1.2.3.4"));
        let ip = resolve_client_ip(peer, &headers, &rules());
        assert_eq!(ip, "203.0.113.9".parse::<IpAddr>().unwrap());
    }

    #[test]
    fn trusted_proxy_uses_rightmost_untrusted() {
        let peer: SocketAddr = "10.0.0.2:443".parse().unwrap();
        let mut headers = http::HeaderMap::new();
        // spoofed left, real client middle, trusted proxy right
        headers.insert(
            "X-Forwarded-For",
            HeaderValue::from_static("8.8.8.8, 203.0.113.50, 10.0.0.2"),
        );
        let ip = resolve_client_ip(peer, &headers, &rules());
        assert_eq!(ip, "203.0.113.50".parse::<IpAddr>().unwrap());
    }

    #[test]
    fn x_real_ip_supported() {
        let mut rules = rules();
        rules[0].header_name = "X-Real-IP".to_owned();
        let peer: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let mut headers = http::HeaderMap::new();
        headers.insert("X-Real-IP", HeaderValue::from_static("198.51.100.7"));
        let ip = resolve_client_ip(peer, &headers, &rules);
        assert_eq!(ip, "198.51.100.7".parse::<IpAddr>().unwrap());
    }
}
