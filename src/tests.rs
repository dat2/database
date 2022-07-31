use crate::table::TABLE_MAX_ROWS;
use anyhow::Result;
use assert_cmd::Command;

#[test]
fn inserts_and_selects() -> Result<()> {
    let mut cmd = Command::cargo_bin("database")?;
    let input = vec!["insert 1 'user' 'user@example.com'", "select", ".exit\n"];
    let output = ["Executed.", "(1, 'user', 'user@example.com')", "Exiting.\n"];
    cmd.write_stdin(input.join("\n"))
        .assert()
        .success()
        .stdout(output.join("\n"));
    Ok(())
}

#[test]
fn fails_at_max_pages() -> Result<()> {
    let mut cmd = Command::cargo_bin("database")?;
    let mut input: Vec<_> = (1..=TABLE_MAX_ROWS + 1)
        .map(|i| format!("insert {i} 'user{i}' 'person{i}@email.com'", i = i))
        .collect();
    input.push(".exit\n".to_owned());
    cmd.write_stdin(input.join("\n"))
        .assert()
        .success()
        .stdout(predicates::str::contains("Table is full."));
    Ok(())
}

#[test]
fn allows_inserting_max_length() -> Result<()> {
    let mut cmd = Command::cargo_bin("database")?;
    let username = "a".repeat(32);
    let email = "a".repeat(255);
    let input = vec![
        format!(
            "insert 1 '{username}' '{email}'    ",
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
    cmd.write_stdin(input.join("\n"))
        .assert()
        .success()
        .stdout(output.join("\n"));
    Ok(())
}
