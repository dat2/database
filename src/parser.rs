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

use crate::table::Row;

pub enum SQL {
    Insert(Row),
    Select,
}

fn decimal(input: &str) -> IResult<&str, u32> {
    map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
        u32::from_str_radix(out, 10)
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
    Ok((input, SQL::Insert(Row::new(id, username, email))))
}

fn select(i: &str) -> IResult<&str, SQL> {
    let (input, _) = tag("select")(i)?;
    Ok((input, SQL::Select))
}

fn sql(i: &str) -> IResult<&str, SQL> {
    alt((insert, select))(i)
}

pub fn parse_sql(line: &str) -> Result<SQL> {
    let (_, output) = sql(line)
        .finish()
        .map_err(|e| anyhow!(format!("Failed to parse statement: {}", e)))?;
    Ok(output)
}
