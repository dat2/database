use anyhow::{Error, Result};

use crate::table::{Row, Table, ROWS_PER_PAGE, ROW_SIZE};

pub struct Cursor<'a> {
    row_num: usize,
    table: &'a mut Table,
}

impl<'a> Cursor<'a> {
    pub fn start(table: &'a mut Table) -> Cursor<'a> {
        Cursor { row_num: 0, table }
    }

    pub fn end(table: &'a mut Table) -> Cursor<'a> {
        Cursor {
            row_num: table.num_rows,
            table,
        }
    }

    fn is_end(&self) -> bool {
        self.row_num == self.table.num_rows
    }

    pub fn value(&mut self) -> Result<&mut [u8]> {
        let page_num = self.row_num / ROWS_PER_PAGE;
        let page = self.table.pager.get_page(page_num)?;
        let row_offset = self.row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;
        Ok(&mut page[byte_offset..byte_offset + ROW_SIZE])
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end() {
            None
        } else {
            let row = self
                .value()
                .and_then(|slot| bincode::deserialize(slot).map_err(Error::msg));
            self.row_num += 1;
            row.ok()
        }
    }
}
