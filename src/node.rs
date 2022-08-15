use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

use crate::table::Row;

#[derive(Serialize, Deserialize, Debug)]
pub enum Type {
    Root,
    Internal,
    Leaf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    key: u32,
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    ty: Type,
    parent: Option<usize>,
    pub cells: Vec<Cell>,
}

impl Node {
    pub fn leaf() -> Node {
        Node {
            ty: Type::Leaf,
            parent: None,
            cells: Vec::new(),
        }
    }

    pub fn num_cells(&self) -> usize {
        self.cells.len()
    }

    pub fn insert(&mut self, key: u32, row: Row) -> Result<()> {
        let value = bincode::serialize(&row)?;
        ensure!(
            self.cells
                .binary_search_by_key(&key, |cell| cell.key)
                .is_err(),
            "Duplicate key exists in the table."
        );
        let idx = self.cells.partition_point(|cell| cell.key < key);
        self.cells.insert(idx, Cell { key, value });
        Ok(())
    }
}
