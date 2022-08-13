use anyhow::{Error, Result};

use crate::table::{Row, Table};

#[derive(Debug)]
pub struct Cursor<'a> {
    page_num: usize,
    cell_num: usize,
    table: &'a mut Table,
}

impl<'a> Cursor<'a> {
    pub fn start(table: &'a mut Table) -> Result<Cursor<'a>> {
        Ok(Cursor {
            page_num: table.root_page_num,
            cell_num: 0,
            table,
        })
    }

    pub fn end(table: &'a mut Table) -> Result<Cursor<'a>> {
        let root_node = table.pager.get_node(table.root_page_num)?;
        Ok(Cursor {
            page_num: table.root_page_num,
            cell_num: root_node.num_cells(),
            table,
        })
    }

    fn is_end(&mut self) -> Result<bool> {
        let root_node = self.table.pager.get_node(self.table.root_page_num)?;
        Ok(root_node.num_cells() == 0)
    }

    pub fn value(&mut self) -> Result<&mut [u8]> {
        let node = self.table.pager.get_node(self.page_num)?;
        let cell = &mut node.cells[self.cell_num];
        Ok(&mut cell.value)
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        match self.is_end() {
            Ok(true) | Err(_) => None,
            Ok(false) => {
                let row = self
                    .value()
                    .and_then(|slot| bincode::deserialize(slot).map_err(Error::msg));
                self.cell_num += 1;
                row.ok()
            }
        }
    }
}
