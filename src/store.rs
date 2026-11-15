use crate::hosts;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    pub domain: String,
    pub blocked: bool,
    #[serde(default)]
    pub locked: bool,
}

pub fn set_unlocked_sites(sites: &mut [Site], blocked: bool) {
    for site in sites.iter_mut().filter(|site| !site.locked) {
        site.blocked = blocked;
    }
}

pub fn path() -> std::path::PathBuf {
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            #[cfg(target_os = "windows")]
            {
                std::env::var_os("APPDATA").map(std::path::PathBuf::from)
            }
            #[cfg(target_os = "macos")]
            {
                std::env::var_os("HOME")
                    .map(|h| std::path::PathBuf::from(h).join("Library/Application Support"))
            }
            #[cfg(target_os = "linux")]
            {
                std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config"))
            }
        })
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()));
    config.join("host-manager/sites.json")
}

pub fn load_hide_locked() -> bool {
    path()
        .parent()
        .and_then(|p| std::fs::read_to_string(p.join("view.json")).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(false)
}

pub fn save_hide_locked(hide: bool) -> std::io::Result<()> {
    let config = path();
    let parent = config.parent().unwrap();
    std::fs::create_dir_all(parent)?;
    std::fs::write(parent.join("view.json"), serde_json::to_vec(&hide).unwrap())
}

pub fn load() -> Vec<Site> {
    let mut sites: Vec<Site> = std::fs::read(path())
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default();
    if let Ok(text) = std::fs::read_to_string(hosts::hosts_path()) {
        for domain in hosts::managed_domains(&text) {
            if !sites.iter().any(|s| s.domain == domain) {
                sites.push(Site {
                    domain,
                    blocked: true,
                    locked: false,
                });
            }
        }
    }
    sites.sort_by(|a, b| a.domain.cmp(&b.domain));
    sites
}

pub fn save(sites: &[Site]) -> std::io::Result<()> {
    let path = path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_vec_pretty(sites).unwrap())?;
    std::fs::rename(tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulk_state_change_skips_locked_sites() {
        let mut sites = vec![
            Site {
                domain: "locked.example".into(),
                blocked: false,
                locked: true,
            },
            Site {
                domain: "open.example".into(),
                blocked: false,
                locked: false,
            },
        ];
        set_unlocked_sites(&mut sites, true);
        assert!(!sites[0].blocked);
        assert!(sites[1].blocked);
        set_unlocked_sites(&mut sites, false);
        assert!(!sites[0].blocked);
        assert!(!sites[1].blocked);
    }

    #[test]
    fn old_site_data_defaults_to_unlocked() {
        let site: Site =
            serde_json::from_str(r#"{"domain":"example.com","blocked":true}"#).unwrap();
        assert!(site.blocked);
        assert!(!site.locked);
    }

    #[test]
    fn locked_state_round_trips() {
        let site = Site {
            domain: "example.com".into(),
            blocked: true,
            locked: true,
        };
        let decoded: Site = serde_json::from_slice(&serde_json::to_vec(&site).unwrap()).unwrap();
        assert!(decoded.locked);
        assert_eq!(decoded.domain, site.domain);
    }
}
