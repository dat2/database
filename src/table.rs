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
pub struct Row<'a> {
    id: u32,
    username: &'a str,
    email: &'a str,
}

impl<'a> Row<'a> {
    pub fn new(id: u32, username: &'a str, email: &'a str) -> Row<'a> {
        Row {
            id,
            username,
            email,
        }
    }
}

pub struct Table {
    pages: Vec<[u8; PAGE_SIZE as usize]>,
    num_rows: u32,
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
        while self.pages.len() < page_num {
            self.pages.push([0; PAGE_SIZE]);
        }
        let page = self.pages[page_num];
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
}
