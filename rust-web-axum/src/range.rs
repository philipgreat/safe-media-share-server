
//! HTTP Range request parsing and byte-window computation (RFC 7233 subset).

/// Inclusive byte range to serve: `start..=end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteRange {
    pub start: u64,
    pub end: u64,
}

impl ByteRange {
    pub fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    pub fn content_range_header(&self, total_len: u64) -> String {
        format!("bytes {}-{}/{}", self.start, self.end, total_len)
    }
}

/// Parse a `Range` header value for a resource of `total_len` bytes.
///
/// Supports `bytes=start-end`, `bytes=start-`, `bytes=-suffix`.
/// Returns `Ok(None)` when the header is absent or not a byte range (caller serves 200).
/// Returns `Err(())` when the range is syntactically valid but unsatisfiable (caller serves 416).
pub fn parse_range_header(header: Option<&str>, total_len: u64) -> Result<Option<ByteRange>, ()> {
    let Some(raw) = header else {
        return Ok(None);
    };
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(None);
    }
    let Some(spec) = raw.strip_prefix("bytes=") else {
        // Unknown unit: ignore Range and serve full body.
        return Ok(None);
    };
    // Multi-range is rare for video players; serve the first range only.
    let first = spec.split(',').next().unwrap_or("").trim();
    if first.is_empty() {
        return Err(());
    }
    let Some((start_s, end_s)) = first.split_once('-') else {
        return Err(());
    };
    let start_s = start_s.trim();
    let end_s = end_s.trim();

    if start_s.is_empty() {
        // suffix: last N bytes
        let suffix: u64 = end_s.parse().map_err(|_| ())?;
        if suffix == 0 || total_len == 0 {
            return Err(());
        }
        let take = suffix.min(total_len);
        return Ok(Some(ByteRange {
            start: total_len - take,
            end: total_len.saturating_sub(1),
        }));
    }

    let start: u64 = start_s.parse().map_err(|_| ())?;
    if start >= total_len {
        return Err(());
    }
    let end = if end_s.is_empty() {
        total_len.saturating_sub(1)
    } else {
        let end: u64 = end_s.parse().map_err(|_| ())?;
        if end < start {
            return Err(());
        }
        end.min(total_len.saturating_sub(1))
    };
    Ok(Some(ByteRange { start, end }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_range_serves_full() {
        assert_eq!(parse_range_header(None, 100), Ok(None));
        assert_eq!(parse_range_header(Some("items=0-1"), 100), Ok(None));
    }

    #[test]
    fn closed_range() {
        assert_eq!(
            parse_range_header(Some("bytes=0-99"), 1000),
            Ok(Some(ByteRange { start: 0, end: 99 }))
        );
    }

    #[test]
    fn open_ended_range() {
        assert_eq!(
            parse_range_header(Some("bytes=500-"), 1000),
            Ok(Some(ByteRange { start: 500, end: 999 }))
        );
    }

    #[test]
    fn suffix_range() {
        assert_eq!(
            parse_range_header(Some("bytes=-100"), 1000),
            Ok(Some(ByteRange { start: 900, end: 999 }))
        );
    }

    #[test]
    fn unsatisfiable_when_start_beyond_len() {
        assert_eq!(parse_range_header(Some("bytes=1000-"), 1000), Err(()));
    }

    #[test]
    fn multi_range_uses_first() {
        assert_eq!(
            parse_range_header(Some("bytes=0-9,20-29"), 100),
            Ok(Some(ByteRange { start: 0, end: 9 }))
        );
    }
}
