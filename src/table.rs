use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

const ID_SIZE: usize = std::mem::size_of::<u32>();
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

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
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, '{}', '{}')", self.id, self.username, self.email)
    }
}

pub struct Cursor<'a> {
    current: usize,
    table: &'a mut Table,
}

impl<'a> Iterator for Cursor<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.table.num_rows {
            None
        } else {
            let row_slot = self.table.row_slot(self.current);
            self.current += 1;
            match bincode::deserialize(row_slot) {
                Ok(row) => Some(row),
                Err(_) => None,
            }
        }
    }
}

pub struct Table {
    pages: Vec<[u8; PAGE_SIZE as usize]>,
    num_rows: usize,
}

impl Table {
    pub fn new() -> Table {
        Table {
            pages: vec![],
            num_rows: 0,
        }
    }

    fn row_slot(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        while self.pages.len() <= page_num {
            self.pages.push([0; PAGE_SIZE]);
        }
        let row_offset = row_num % ROWS_PER_PAGE;
        let offset = row_offset * ROW_SIZE;
        &mut self.pages[page_num][offset..offset + ROW_SIZE]
    }

    pub fn insert(&mut self, row: Row) -> Result<()> {
        if self.num_rows as usize >= TABLE_MAX_ROWS {
            bail!("Table is full.");
        }
        let row_slot = self.row_slot(self.num_rows as usize);
        bincode::serialize_into(row_slot, &row)?;
        self.num_rows += 1;
        Ok(())
    }

    pub fn select(&mut self) -> Cursor {
        Cursor {
            current: 0,
            table: self,
        }
    }
}
