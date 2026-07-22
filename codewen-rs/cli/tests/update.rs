use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn codewen_command(codewen_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(codewen_utils_cargo_bin::cargo_bin("codewen")?);
    cmd.env("CODEWEN_HOME", codewen_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let codewen_home = TempDir::new()?;

    codewen_command(codewen_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains("`codewen update` is not available in debug builds"));

    Ok(())
}
