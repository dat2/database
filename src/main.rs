#![feature(box_patterns)]
mod cursor;
mod database;
mod node;
mod pager;
mod table;
#[cfg(test)]
mod tests;

use anyhow::{anyhow, bail, Result};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use sqlparser::ast::{
    Expr, Ident, ObjectName, Query, Select, SetExpr, Statement, TableFactor, TableWithJoins, Value,
    Values,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;
use table::Row;

use crate::cursor::Cursor;
use crate::database::{close, open};
use crate::table::Table;

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

fn is_select(ast: &[Statement]) -> bool {
    match ast {
        [Statement::Query(box Query {
            body: box SetExpr::Select(box Select { from, .. }),
            ..
        })] => match from.as_slice() {
            [TableWithJoins {
                relation:
                    TableFactor::Table {
                        name: ObjectName(names),
                        ..
                    },
                ..
            }] => match names.as_slice() {
                [Ident { value, .. }] => match value.as_str() {
                    "users" => true,
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

fn is_insert(ast: &[Statement]) -> Option<Row> {
    match ast {
        [Statement::Insert {
            table_name: ObjectName(table_name),
            source:
                box Query {
                    body: box SetExpr::Values(Values(values)),
                    ..
                },
            ..
        }] => match table_name.as_slice() {
            [Ident { value, .. }] => match value.as_str() {
                "users" => match values[0].as_slice() {
                    [Expr::Value(Value::Number(id, false)), Expr::Value(Value::SingleQuotedString(username)), Expr::Value(Value::SingleQuotedString(email))] => {
                        Some(Row::new(id.parse().unwrap(), username, email))
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn execute_sql(table: Rc<RefCell<Table>>, line: &str) -> Result<()> {
    let ast = Parser::parse_sql(&GenericDialect {}, line)?;
    if is_select(&ast) {
        let mut table = table.borrow_mut();
        let cursor = Cursor::start(&mut table)?;
        for row in cursor {
            println!("{}", row);
        }
        Ok(())
    } else if let Some(row) = is_insert(&ast) {
        row.validate()?;
        let mut table = table.borrow_mut();
        table.insert(row)
    } else {
        Err(anyhow!("Unimplemented"))
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
    if let Err(err) = execute(table, &line) {
        println!("Error: {}", err);
    }
    Ok(())
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
