use anyhow::{anyhow, ensure, Result};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use crate::node::Node;

const PAGE_SIZE: usize = 4096;
const MAX_PAGES: usize = 100;

#[derive(Debug)]
pub struct Pager {
    file: File,
    num_pages: usize,
    nodes: HashMap<usize, Node>,
}

impl Pager {
    pub fn new(filename: &str) -> Result<Pager> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(filename)?;
        let metadata = file.metadata()?;
        let num_pages = metadata.len() as usize / PAGE_SIZE;
        Ok(Pager {
            file,
            num_pages,
            nodes: HashMap::new(),
        })
    }

    pub fn empty(&self) -> bool {
        self.num_pages == 0
    }

    pub fn set_node(&mut self, page_num: usize, node: Node) {
        self.nodes.insert(page_num, node);
    }

    pub fn get_node(&mut self, page_num: usize) -> Result<&mut Node> {
        ensure!(page_num <= MAX_PAGES, "Page number out of bounds.");
        if self.num_pages < page_num {
            self.num_pages = page_num;
        }
        if !self.nodes.contains_key(&page_num) {
            let mut buffer = vec![0; PAGE_SIZE];
            self.file
                .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))?;
            self.file.read(&mut buffer)?;
            let node: Node = bincode::deserialize(&buffer)?;
            self.nodes.insert(page_num, node);
        }
        self.nodes
            .get_mut(&page_num)
            .ok_or(anyhow!("Node not initialized"))
    }

    pub fn flush(&mut self) -> Result<()> {
        for (page_num, node) in self.nodes.iter() {
            self.file
                .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))?;
            let mut buffer = vec![0; PAGE_SIZE];
            bincode::serialize_into(&mut buffer, node)?;
            self.file.write(&buffer)?;
        }
        Ok(())
    }
}
