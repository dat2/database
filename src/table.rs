use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use crate::pager::Pager;
use anyhow::{bail, ensure, Error, Result};
use serde::{Deserialize, Serialize};

const ID_SIZE: usize = std::mem::size_of::<u32>();
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

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
            let (page_num, slot) = self.table.get_page_and_row_slot(self.current);
            let mut pager = self.table.pager.borrow_mut();
            self.current += 1;
            pager
                .get_page(page_num)
                .and_then(|page| bincode::deserialize(&page[slot]).map_err(Error::msg))
                .ok()
        }
    }
}

pub struct Table {
    pub num_rows: usize,
    pub pager: Rc<RefCell<Pager>>,
}

impl Table {
    pub fn new(pager: Rc<RefCell<Pager>>) -> Result<Table> {
        let num_rows = pager.borrow().get_file_size()? as usize / ROW_SIZE;
        Ok(Table { num_rows, pager })
    }

    fn get_page_and_row_slot(&mut self, row_num: usize) -> (usize, Range<usize>) {
        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = row_num % ROWS_PER_PAGE;
        let offset = row_offset * ROW_SIZE;
        (page_num, offset..offset + ROW_SIZE)
    }

    pub fn insert(&mut self, row: Row) -> Result<()> {
        if self.num_rows as usize >= TABLE_MAX_ROWS {
            bail!("Table is full.");
        }
        let (page_num, slot) = self.get_page_and_row_slot(self.num_rows);
        let mut pager = self.pager.borrow_mut();
        let page = pager.get_page(page_num)?;
        bincode::serialize_into(&mut page[slot], &row)?;
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
