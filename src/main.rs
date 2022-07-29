use anyhow::{anyhow, Result};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace1, one_of},
    combinator::{map_res, recognize},
    multi::many1,
    sequence::{delimited, tuple},
    Finish, IResult,
};
use rustyline::error::ReadlineError;
use rustyline::Editor;

enum SQL<'a> {
    Insert(u64, &'a str, &'a str),
    Select,
}

fn decimal(input: &str) -> IResult<&str, u64> {
    map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
        u64::from_str_radix(out, 10)
    })(input)
}

fn insert(i: &str) -> IResult<&str, SQL> {
    let (input, _) = tag("insert")(i)?;
    let (input, (_, id, _, username, _, email)) = tuple((
        multispace1,
        decimal,
        multispace1,
        delimited(char('\''), is_not("'"), char('\'')),
        multispace1,
        delimited(char('\''), is_not("'"), char('\'')),
    ))(input)?;
    Ok((input, SQL::Insert(id, username, email)))
}

fn select(i: &str) -> IResult<&str, SQL> {
    let (input, _) = tag("select")(i)?;
    Ok((input, SQL::Select))
}

fn sql(i: &str) -> IResult<&str, SQL> {
    alt((insert, select))(i)
}

struct Row<'a> {
    id: usize,
    email: &'a str,
    username: &'a str,
}


fn execute_command(line: &str) -> Result<()> {
    match line {
        ".exit" => {
            println!("Exiting.");
            std::process::exit(0);
        }
        _ => {
            println!("Unrecognized command: {}", line);
            Ok(())
        }
    }
}

fn parse_sql(line: &str) -> Result<SQL> {
    let (_, output) = sql(line)
        .finish()
        .map_err(|e| anyhow!(format!("Failed to parse statement: {}", e)))?;
    Ok(output)
}

fn execute_sql(line: &str) -> Result<()> {
    let sql = parse_sql(line)?;
    match sql {
        SQL::Select => {
            println!("Selecting...");
            Ok(())
        }
        SQL::Insert(id, email, username) => {
            println!("Inserting {} {} {}", id, email, username);
            Ok(())
        }
    }
}

fn process(line: &str) -> Result<()> {
    if let Some(char) = line.chars().nth(0) {
        if char == '.' {
            return execute_command(line);
        }
    }
    execute_sql(line)
}

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline("database > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if let Err(e) = process(&line) {
                    println!("Error: {:?}", e);
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
