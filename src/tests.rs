use anyhow::Result;
use assert_cmd::Command;

#[test]
fn inserts_and_selects() -> Result<()> {
    let mut cmd = Command::cargo_bin("database")?;
    let input = vec!["insert 1 'user' 'user@example.com'", "select", ".exit\n"];
    let output = ["Executed.", "(1, 'user', 'user@example.com')", "Exiting.\n"];
    let file = assert_fs::NamedTempFile::new("database.db")?;
    cmd.arg(file.path())
        .write_stdin(input.join("\n"))
        .assert()
        .success()
        .stdout(output.join("\n"));
    Ok(())
}

#[test]
fn allows_inserting_max_length() -> Result<()> {
    let mut cmd = Command::cargo_bin("database")?;
    let username = "a".repeat(32);
    let email = "a".repeat(255);
    let input = vec![
        format!(
            "insert 1 '{username}' '{email}'",
            username = username,
            email = email,
        ),
        "select".to_owned(),
        ".exit\n".to_owned(),
    ];
    let output = [
        "Executed.".to_owned(),
        format!(
            "(1, '{username}', '{email}')",
            username = username,
            email = email
        ),
        "Exiting.\n".to_owned(),
    ];

    let file = assert_fs::NamedTempFile::new("database.db")?;
    cmd.arg(file.path())
        .write_stdin(input.join("\n"))
        .assert()
        .success()
        .stdout(output.join("\n"));
    Ok(())
}
