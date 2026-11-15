use crate::store::Site;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn hosts_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return std::env::var_os("SystemRoot")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(r"C:\Windows"))
            .join(r"System32\drivers\etc\hosts");
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        PathBuf::from("/etc/hosts")
    }
}

pub fn managed_domains(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let mut fields = line.split('#').next().unwrap_or("").split_whitespace();
        if fields.next() != Some("127.0.0.1") {
            continue;
        }
        for host in fields {
            let domain = host.strip_prefix("www.").unwrap_or(host);
            if host.starts_with("www.") || host.contains('.') {
                if !out.iter().any(|d| d == domain) {
                    out.push(domain.to_string());
                }
            }
        }
    }
    out
}

fn valid_domain(domain: &str) -> bool {
    domain.len() <= 253
        && domain.split('.').count() >= 2
        && domain.split('.').all(|part| {
            !part.is_empty()
                && part.len() <= 63
                && part.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-')
                && part.as_bytes()[0].is_ascii_alphanumeric()
                && part.as_bytes()[part.len() - 1].is_ascii_alphanumeric()
        })
}

pub fn apply(sites: &[Site]) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let mut child = Command::new("pkexec")
            .arg(std::env::current_exe().map_err(|e| e.to_string())?)
            .arg("--apply")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Avvio pkexec: {e}"))?;
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&serde_json::to_vec(sites).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let output = child.wait_with_output().map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        let path = write_elevation_payload(sites)?;
        let result = elevated_apply(&std::env::current_exe().map_err(|e| e.to_string())?, &path);
        let _ = std::fs::remove_file(path);
        result
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn write_elevation_payload(sites: &[Site]) -> Result<PathBuf, String> {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let path =
        std::env::temp_dir().join(format!("host-manager-{}-{stamp}.json", std::process::id()));
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| e.to_string())?;
    file.write_all(&serde_json::to_vec(sites).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(path)
}

#[cfg(target_os = "macos")]
fn elevated_apply(exe: &Path, payload: &Path) -> Result<(), String> {
    let command = format!("{} --apply-file {}", shell_quote(exe), shell_quote(payload));
    let script = format!(
        "do shell script \"{}\" with administrator privileges",
        command.replace('\\', "\\\\").replace('"', "\\\"")
    );
    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|e| e.to_string())?;
    command_result(
        output.status.success(),
        &output.stderr,
        "Autorizzazione amministratore annullata",
    )
}

#[cfg(target_os = "macos")]
fn shell_quote(path: &Path) -> String {
    format!("'{}'", path.to_string_lossy().replace('\'', "'\\''"))
}

#[cfg(target_os = "windows")]
fn elevated_apply(exe: &Path, payload: &Path) -> Result<(), String> {
    let exe = powershell_quote(exe);
    let payload = powershell_quote(payload);
    let script = format!("$p = Start-Process -FilePath {exe} -ArgumentList '--apply-file \"{payload}\"' -Verb RunAs -Wait -PassThru; exit $p.ExitCode");
    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .map_err(|e| e.to_string())?;
    command_result(
        output.status.success(),
        &output.stderr,
        "Richiesta UAC annullata",
    )
}

#[cfg(target_os = "windows")]
fn powershell_quote(path: &Path) -> String {
    format!("'{}'", path.to_string_lossy().replace('\'', "''"))
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn command_result(success: bool, stderr: &[u8], denied: &str) -> Result<(), String> {
    if success {
        Ok(())
    } else {
        let message = String::from_utf8_lossy(stderr).trim().to_string();
        Err(if message.is_empty() {
            denied.to_string()
        } else {
            message
        })
    }
}

pub fn apply_from_stdin() -> Result<(), String> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    let sites: Vec<Site> = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    apply_sites(&sites)
}

pub fn apply_from_file(path: &Path) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let sites: Vec<Site> = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    apply_sites(&sites)
}

fn apply_sites(sites: &[Site]) -> Result<(), String> {
    let hosts = hosts_path();
    let text = std::fs::read_to_string(&hosts).map_err(|e| e.to_string())?;
    let output = render_hosts(&text, &sites)?;
    let tmp = hosts.with_extension("host-manager.tmp");
    std::fs::write(&tmp, output).map_err(|e| e.to_string())?;
    std::fs::rename(tmp, hosts).map_err(|e| e.to_string())?;
    Ok(())
}

fn render_hosts(text: &str, sites: &[Site]) -> Result<String, String> {
    if sites.iter().any(|s| !valid_domain(&s.domain)) {
        return Err("Dominio non valido".into());
    }
    let mut lines: Vec<String> = text
        .lines()
        .filter_map(|line| {
            let (body, comment) = line
                .split_once('#')
                .map_or((line, None), |(a, b)| (a, Some(b)));
            let mut f = body.split_whitespace();
            let ip = f.next().unwrap_or("");
            if ip != "127.0.0.1" {
                return Some(line.to_string());
            }
            let mut names: Vec<_> = f
                .filter(|name| {
                    !sites
                        .iter()
                        .any(|s| *name == s.domain || *name == format!("www.{}", s.domain))
                })
                .collect();
            if names.is_empty() {
                return None;
            }
            let mut kept = format!("{ip}\t{}", names.join("\t"));
            if let Some(comment) = comment {
                kept.push_str(" #");
                kept.push_str(comment);
            }
            names.clear();
            Some(kept)
        })
        .collect();
    for site in sites.iter().filter(|s| s.blocked) {
        lines.push(format!("127.0.0.1\t{}", site.domain));
        lines.push(format!("127.0.0.1\twww.{}", site.domain));
    }
    let mut output = lines.join("\n");
    output.push('\n');
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn site(domain: &str, blocked: bool) -> Site {
        Site {
            domain: domain.into(),
            blocked,
            locked: false,
        }
    }

    #[test]
    fn blocks_domain_and_www_and_preserves_unrelated_hosts_and_comments() {
        let original = "127.0.0.1 localhost facebook.com # local aliases\n192.0.2.3 keep.example\n";
        let result = render_hosts(original, &[site("facebook.com", true)]).unwrap();
        assert!(result.contains("127.0.0.1\tlocalhost # local aliases"));
        assert!(result.contains("192.0.2.3 keep.example"));
        assert!(result.contains("127.0.0.1\tfacebook.com\n"));
        assert!(result.contains("127.0.0.1\twww.facebook.com\n"));
        assert_eq!(result.matches("facebook.com").count(), 2);
    }

    #[test]
    fn unblocking_removes_only_managed_names() {
        let original = "127.0.0.1 facebook.com www.facebook.com localhost # keep me\n127.0.0.1 other.example\n";
        let result = render_hosts(original, &[site("facebook.com", false)]).unwrap();
        assert!(!result.contains("facebook.com"));
        assert!(result.contains("localhost # keep me"));
        assert!(result.contains("other.example"));
    }

    #[test]
    fn rejects_invalid_domains_without_rendering_hosts() {
        for invalid in [
            "",
            "facebook.com/path",
            "-bad.example",
            "bad..example",
            "bad.example-",
        ] {
            assert!(
                render_hosts("127.0.0.1 localhost\n", &[site(invalid, true)]).is_err(),
                "{invalid}"
            );
        }
    }

    #[test]
    fn imports_domain_and_www_as_one_site() {
        let domains = managed_domains(
            "127.0.0.1 facebook.com\n127.0.0.1 www.facebook.com\n127.0.0.1 localhost\n",
        );
        assert_eq!(domains, ["facebook.com"]);
    }
}
