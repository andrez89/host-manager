use serde::{Deserialize, Serialize};
use crate::hosts;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    pub domain: String,
    pub blocked: bool,
}

pub fn path() -> std::path::PathBuf {
    std::env::var_os("XDG_CONFIG_HOME").map(std::path::PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config")))
        .unwrap_or_else(|| ".".into()).join("host-manager/sites.json")
}

pub fn load() -> Vec<Site> {
    let mut sites: Vec<Site> = std::fs::read(path()).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
    if let Ok(text) = std::fs::read_to_string("/etc/hosts") {
        for domain in hosts::managed_domains(&text) {
            if !sites.iter().any(|s| s.domain == domain) { sites.push(Site { domain, blocked: true }); }
        }
    }
    sites.sort_by(|a,b| a.domain.cmp(&b.domain));
    sites
}

pub fn save(sites: &[Site]) -> std::io::Result<()> {
    let path = path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_vec_pretty(sites).unwrap())?;
    std::fs::rename(tmp, path)
}
