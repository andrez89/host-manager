use crate::store::Site;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub fn managed_domains(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let mut fields = line.split('#').next().unwrap_or("").split_whitespace();
        if fields.next() != Some("127.0.0.1") { continue; }
        for host in fields {
            let domain = host.strip_prefix("www.").unwrap_or(host);
            if host.starts_with("www.") || host.contains('.') {
                if !out.iter().any(|d| d == domain) { out.push(domain.to_string()); }
            }
        }
    }
    out
}

fn valid_domain(domain: &str) -> bool {
    domain.len() <= 253 && domain.split('.').count() >= 2 && domain.split('.').all(|part| {
        !part.is_empty() && part.len() <= 63 && part.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-') &&
        part.as_bytes()[0].is_ascii_alphanumeric() && part.as_bytes()[part.len()-1].is_ascii_alphanumeric()
    })
}

pub fn apply(sites: &[Site]) -> Result<(), String> {
    let mut child = Command::new("pkexec").arg(std::env::current_exe().map_err(|e| e.to_string())?).arg("--apply")
        .stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped()).spawn().map_err(|e| format!("Avvio pkexec: {e}"))?;
    child.stdin.take().unwrap().write_all(&serde_json::to_vec(sites).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    if output.status.success() { Ok(()) } else { Err(String::from_utf8_lossy(&output.stderr).trim().to_string()) }
}

pub fn apply_from_stdin() -> Result<(), String> {
    let mut bytes = Vec::new(); std::io::stdin().read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    let sites: Vec<Site> = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if sites.iter().any(|s| !valid_domain(&s.domain)) { return Err("Dominio non valido".into()); }
    let text = std::fs::read_to_string("/etc/hosts").map_err(|e| e.to_string())?;
    let mut lines: Vec<String> = text.lines().filter_map(|line| {
        let (body, comment) = line.split_once('#').map_or((line, None), |(a,b)| (a, Some(b)));
        let mut f = body.split_whitespace();
        let ip = f.next().unwrap_or("");
        if ip != "127.0.0.1" { return Some(line.to_string()); }
        let mut names: Vec<_> = f.filter(|name| !sites.iter().any(|s| *name == s.domain || *name == format!("www.{}", s.domain))).collect();
        if names.is_empty() { return None; }
        let mut kept = format!("{ip}\t{}", names.join("\t"));
        if let Some(comment) = comment { kept.push_str(" #"); kept.push_str(comment); }
        names.clear();
        Some(kept)
    }).collect();
    for site in sites.iter().filter(|s| s.blocked) {
        lines.push(format!("127.0.0.1\t{}", site.domain));
        lines.push(format!("127.0.0.1\twww.{}", site.domain));
    }
    let mut output = lines.join("\n"); output.push('\n');
    let tmp = "/etc/hosts.host-manager.tmp";
    std::fs::write(tmp, output).map_err(|e| e.to_string())?;
    std::fs::rename(tmp, "/etc/hosts").map_err(|e| e.to_string())?;
    Ok(())
}
