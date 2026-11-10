use assert_cmd::Command;
use predicates::prelude::*;

fn simaster() -> Command {
    Command::new(env!("CARGO_BIN_EXE_simaster"))
}

#[test]
fn help_lists_top_level_commands() {
    simaster()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("auth"))
        .stdout(predicate::str::contains("profile"))
        .stdout(predicate::str::contains("doctor"))
        .stdout(predicate::str::contains("completions"));
}

#[test]
fn doctor_json_has_stable_envelope_and_version() {
    let output = simaster()
        .args(["doctor", "--json"])
        .output()
        .expect("doctor should run");

    assert!(output.status.success(), "doctor failed: {output:?}");
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("doctor output must be valid JSON");

    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["data"]["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(value["data"]["auth"]["status"], "not_configured");
    assert!(value["data"]["auth"]["provider"].is_null());
    assert!(value["meta"]["generated_at_unix"].is_u64());
}

#[test]
fn auth_login_reports_not_implemented() {
    simaster()
        .args(["auth", "login", "--username", "test"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not implemented"));
}

#[test]
fn completions_bash_is_generated() {
    simaster()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("simaster"));
}
