use anyhow::Result;
use std::fs::{File, OpenOptions};

const PAGE_SIZE: usize = 4096;

pub struct Pager {
    file: File,
    pages: Vec<[u8; PAGE_SIZE as usize]>,
}

impl Pager {
    pub fn new(filename: &str) -> Result<Pager> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(filename)?;
        Ok(Pager {
            file,
            pages: vec![],
        })
    }
}
