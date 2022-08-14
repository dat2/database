use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;

use crate::node::Node;
use crate::pager::Pager;
use crate::table::Table;

pub fn open(filename: &str) -> Result<Rc<RefCell<Table>>> {
    let mut pager = Pager::new(filename)?;
    if pager.empty() {
        let page = pager.get_page(0)?;
        bincode::serialize_into(page.as_mut_slice(), &Node::leaf())?;
        pager.flush()?;
    }
    let table = Table::new(pager);
    let table = Rc::new(RefCell::new(table));
    Ok(table)
}

pub fn close(table: Rc<RefCell<Table>>) -> Result<()> {
    let mut table = table.borrow_mut();
    table.flush()?;
    Ok(())
}
