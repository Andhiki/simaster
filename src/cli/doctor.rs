use std::time::{SystemTime, UNIX_EPOCH};

use clap::Args;
use serde::Serialize;

use crate::config;
use crate::error::Error;
use crate::output;

/// Offline environment diagnostics.
#[derive(Debug, Args)]
pub struct Doctor {}

#[derive(Debug, Serialize)]
struct Envelope {
    schema_version: u8,
    data: DoctorData,
    meta: Meta,
}

#[derive(Debug, Serialize)]
struct DoctorData {
    version: &'static str,
    os: &'static str,
    arch: &'static str,
    config_dir: Option<String>,
    auth: AuthState,
    checks: Vec<Check>,
}

#[derive(Debug, Serialize)]
struct AuthState {
    provider: Option<String>,
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct Check {
    name: &'static str,
    status: &'static str,
    detail: String,
}

#[derive(Debug, Serialize)]
struct Meta {
    generated_at_unix: u64,
}

/// Run offline diagnostics and print either human-readable text or JSON.
pub fn run(_doctor: Doctor, json: bool) -> Result<(), Error> {
    let config_dir = config::config_dir();
    let config_dir_display = config_dir
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "unavailable".to_string());

    let checks = vec![
        Check {
            name: "config_dir",
            status: if config_dir.is_some() { "ok" } else { "warn" },
            detail: config_dir_display.clone(),
        },
        Check {
            name: "auth",
            status: "warn",
            detail: "no stored session; auth commands are not implemented".to_string(),
        },
        Check {
            name: "network",
            status: "skipped",
            detail: "offline diagnostics never contact SIMASTER".to_string(),
        },
    ];

    if json {
        let envelope = Envelope {
            schema_version: 1,
            data: DoctorData {
                version: env!("CARGO_PKG_VERSION"),
                os: std::env::consts::OS,
                arch: std::env::consts::ARCH,
                config_dir: config_dir.as_ref().map(|path| path.display().to_string()),
                auth: AuthState {
                    provider: None,
                    status: "not_configured",
                },
                checks,
            },
            meta: Meta {
                generated_at_unix: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|duration| duration.as_secs())
                    .unwrap_or_default(),
            },
        };

        return output::print_json(&envelope);
    }

    println!("simaster doctor");
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!("os: {}", std::env::consts::OS);
    println!("arch: {}", std::env::consts::ARCH);
    println!("config_dir: {config_dir_display}");
    println!("auth: not configured (provider: none)");
    println!("checks:");
    for check in &checks {
        println!("  {:7} {}: {}", check.status, check.name, check.detail);
    }

    Ok(())
}
