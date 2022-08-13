use serde::{Deserialize, Serialize};

use crate::table::{Row, ROWS_PER_PAGE};
use anyhow::Result;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Type {
    Root,
    Internal,
    Leaf,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Cell {
    key: usize,
    pub value: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

    pub fn cell(&mut self, cell_num: usize) -> Option<&mut Cell> {
        self.cells.get_mut(cell_num)
    }

    pub fn insert(&mut self, key: usize, row: Row) -> Result<()> {
        let cell = Cell {
            key,
            value: bincode::serialize(&row)?,
        };
        self.cells.push(cell);
        Ok(())
    }
}
