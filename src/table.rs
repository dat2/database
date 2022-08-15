use std::collections::{hash_map::Entry, HashMap};

use crate::node::Node;
use crate::pager::Pager;
use anyhow::{anyhow, ensure, Result};
use serde::{Deserialize, Serialize};

const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Debug)]
pub struct Table {
    pub root_page_num: usize,
    pager: Pager,
    nodes: HashMap<usize, Node>,
}

impl Table {
    pub fn new(pager: Pager) -> Table {
        Table {
            root_page_num: 0,
            pager,
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, row: Row) -> Result<()> {
        let node = self.get_node(self.root_page_num)?;
        node.insert(row.id, row)
    }

    pub fn get_node(&mut self, page_num: usize) -> Result<&mut Node> {
        if let Entry::Vacant(e) = self.nodes.entry(page_num) {
            let page = self.pager.get_page(page_num)?;
            let node = bincode::deserialize_from(page.as_slice())?;
            e.insert(node);
        }
        self.nodes
            .get_mut(&page_num)
            .ok_or_else(|| anyhow!("Node not initialized"))
    }

    pub fn flush(&mut self) -> Result<()> {
        for (page_num, node) in self.nodes.iter() {
            let page = self.pager.get_page(*page_num)?;
            bincode::serialize_into(page.as_mut_slice(), &node)?;
        }
        self.pager.flush()
    }
}
