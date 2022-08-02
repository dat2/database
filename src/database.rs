use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;

use crate::pager::Pager;
use crate::table::Table;

pub fn open(filename: &str) -> Result<Rc<RefCell<Table>>> {
    let pager = Pager::new(filename)?;
    let table = Table::new(pager)?;
    let table = Rc::new(RefCell::new(table));
    Ok(table)
}

pub fn close(table: Rc<RefCell<Table>>) -> Result<()> {
    let mut table = table.borrow_mut();
    table.pager.flush()?;
    Ok(())
}
