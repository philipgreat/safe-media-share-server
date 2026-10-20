
//! Admin settings (app config, not domain model): external base URL prefix.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SettingsStore {
    path: PathBuf,
    inner: Arc<Mutex<Settings>>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    /// Public/external base URL prefix, e.g. `https://media.example.com`.
    /// Empty means "use the browser origin / local bind address".
    #[serde(default)]
    pub external_base_url: String,
}

impl SettingsStore {
    pub fn open_default() -> Self {
        let path = std::env::var("MEDIA_GUARD_SETTINGS_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("admin-settings.json"));
        Self::open(path)
    }

    pub fn open(path: PathBuf) -> Self {
        let settings = Self::load_file(&path);
        Self {
            path,
            inner: Arc::new(Mutex::new(settings)),
        }
    }

    fn load_file(path: &Path) -> Settings {
        if let Ok(raw) = std::fs::read_to_string(path) {
            if let Ok(s) = serde_json::from_str::<Settings>(&raw) {
                return s;
            }
        }
        // Optional env fallback at first boot.
        let from_env = std::env::var("MEDIA_GUARD_EXTERNAL_URL")
            .or_else(|_| std::env::var("PUBLIC_BASE_URL"))
            .unwrap_or_default();
        Settings {
            external_base_url: normalize_base_url(&from_env),
        }
    }

    pub fn get(&self) -> Settings {
        self.inner.lock().expect("settings lock").clone()
    }

    pub fn set_external_base_url(&self, value: &str) -> Settings {
        let normalized = normalize_base_url(value);
        {
            let mut guard = self.inner.lock().expect("settings lock");
            guard.external_base_url = normalized;
            let to_write = guard.clone();
            if let Some(parent) = self.path.parent() {
                if !parent.as_os_str().is_empty() {
                    let _ = std::fs::create_dir_all(parent);
                }
            }
            if let Ok(raw) = serde_json::to_string_pretty(&to_write) {
                let _ = std::fs::write(&self.path, raw);
            }
        }
        self.get()
    }
}

/// Trim, strip trailing slash, allow empty.
pub fn normalize_base_url(raw: &str) -> String {
    let s = raw.trim();
    if s.is_empty() {
        return String::new();
    }
    // Accept host without scheme by assuming https:// for external URLs.
    let with_scheme = if s.contains("://") {
        s.to_owned()
    } else {
        format!("https://{s}")
    };
    with_scheme.trim_end_matches('/').to_owned()
}

/// Join external prefix (if any) with an app path that starts with `/`.
pub fn join_external(base: &str, path: &str) -> String {
    let path = if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("/{path}")
    };
    let base = normalize_base_url(base);
    if base.is_empty() {
        path
    } else {
        format!("{base}{path}")
    }
}
