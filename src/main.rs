mod database;
mod pager;
mod parser;
mod table;
#[cfg(test)]
mod tests;

use anyhow::{anyhow, bail, Result};
use std::cell::RefCell;
use std::rc::Rc;

use crate::database::{close, open};
use crate::parser::{parse_sql, SQL};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use table::Table;

fn execute_command(table: Rc<RefCell<Table>>, line: &str) -> Result<()> {
    match line {
        ".exit" => {
            println!("Exiting.");
            close(table)?;
            std::process::exit(0);
        }
        _ => {
            println!("Unrecognized command: {}", line);
            Ok(())
        }
    }
}

fn prepare_sql(line: &str) -> Result<SQL> {
    let sql = parse_sql(line)?;
    if let SQL::Insert(ref row) = sql {
        row.validate()?;
    }
    Ok(sql)
}

fn execute_sql(table: Rc<RefCell<Table>>, line: &str) -> Result<()> {
    let sql = prepare_sql(line)?;
    match sql {
        SQL::Select => {
            for row in table.borrow_mut().select() {
                println!("{}", row);
            }
            Ok(())
        }
        SQL::Insert(row) => {
            table.borrow_mut().insert(row)?;
            println!("Executed.");
            Ok(())
        }
    }
}

fn execute(table: Rc<RefCell<Table>>, line: &str) -> Result<()> {
    if let Some(char) = line.chars().nth(0) {
        if char == '.' {
            return execute_command(table, line);
        }
    }
    execute_sql(table, line)
}

fn run(table: Rc<RefCell<Table>>, rl: &mut Editor<()>) -> Result<()> {
    let line = rl.readline("database > ").map_err(|err| match err {
        ReadlineError::Interrupted | ReadlineError::Eof => anyhow!("Exiting."),
        e => anyhow!("Unexpected error: {}", e),
    })?;
    rl.add_history_entry(&line);
    execute(table, &line)
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        bail!("Must supply a db filename.");
    }
    let table = open(&args[1])?;
    let mut rl = Editor::<()>::new()?;
    loop {
        run(table.clone(), &mut rl)?;
    }
}
