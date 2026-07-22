use std::path::Path;

use anyhow::Result;
use app_test_support::app_server_json_shutdown_event;
use predicates::str::contains;
use pretty_assertions::assert_eq;
use serde_json::json;
use tempfile::TempDir;

fn codewen_command(codewen_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(codewen_utils_cargo_bin::cargo_bin("codewen")?);
    cmd.env("CODEWEN_HOME", codewen_home);
    Ok(cmd)
}

#[test]
fn strict_config_rejects_unknown_config_fields_for_app_server() -> Result<()> {
    let codewen_home = TempDir::new()?;
    std::fs::write(
        codewen_home.path().join("config.toml"),
        r#"
foo = "bar"
"#,
    )?;

    let mut cmd = codewen_command(codewen_home.path())?;
    cmd.args(["app-server", "--strict-config", "--listen", "off"])
        .assert()
        .failure()
        .stderr(contains("unknown configuration field"));

    Ok(())
}

#[test]
fn app_server_emits_json_info_events() -> Result<()> {
    let codewen_home = TempDir::new()?;
    let event = app_server_json_shutdown_event("codewen", &["app-server"], codewen_home.path())?;

    assert_eq!(
        event,
        json!({
            "level": "INFO",
            "fields": {
                "message": "processor task exited",
                "exit_reason": "last_connection_closed",
                "remaining_connection_count": 0,
                "shutdown_forced": false,
            },
            "target": "codewen_app_server",
        })
    );

    Ok(())
}
