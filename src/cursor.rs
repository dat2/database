use anyhow::{ensure, Result};

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

    fn advance(&mut self) -> Result<Row> {
        let node = self.table.get_node(self.page_num)?;
        ensure!(self.cell_num < node.num_cells(), "Exhausted");
        let cell = &mut node.cells[self.cell_num];
        self.cell_num += 1;
        let row = bincode::deserialize_from(cell.value.as_slice())?;
        Ok(row)
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance().ok()
    }
}
