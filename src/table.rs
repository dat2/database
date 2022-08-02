use crate::cursor::Cursor;
use crate::pager::Pager;
use anyhow::{bail, ensure, Result};
use serde::{Deserialize, Serialize};

const ID_SIZE: usize = std::mem::size_of::<u32>();
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;
pub const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
pub const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: u32, username: &str, email: &str) -> Row {
        Row {
            id,
            username: username.to_owned(),
            email: email.to_owned(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.username.len() < USERNAME_SIZE,
            "Column (username) is too long."
        );
        ensure!(self.email.len() < EMAIL_SIZE, "Column (email) is too long.");
        Ok(())
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, '{}', '{}')", self.id, self.username, self.email)
    }
}

pub struct Table {
    pub num_rows: usize,
    pub pager: Pager,
}

impl Table {
    pub fn new(pager: Pager) -> Result<Table> {
        let num_rows = pager.get_file_size()? as usize / ROW_SIZE;
        Ok(Table { num_rows, pager })
    }

    pub fn insert(&mut self, row: Row) -> Result<()> {
        if self.num_rows as usize >= TABLE_MAX_ROWS {
            bail!("Table is full.");
        }
        let mut cursor = Cursor::end(self);
        let slot = cursor.value()?;
        bincode::serialize_into(slot, &row)?;
        self.num_rows += 1;
        Ok(())
    }
}
