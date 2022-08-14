use anyhow::{anyhow, ensure, Result};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

const PAGE_SIZE: usize = 4096;
const MAX_PAGES: usize = 100;

#[derive(Debug)]
pub struct Pager {
    file: File,
    num_pages: usize,
    pages: HashMap<usize, Vec<u8>>,
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
            pages: HashMap::new(),
        })
    }

    pub fn empty(&self) -> bool {
        self.num_pages == 0
    }

    pub fn get_page(&mut self, page_num: usize) -> Result<&mut Vec<u8>> {
        ensure!(page_num <= MAX_PAGES, "Page number out of bounds.");
        if self.num_pages < page_num {
            self.num_pages = page_num;
        }
        if let Entry::Vacant(e) = self.pages.entry(page_num) {
            let mut buffer = vec![0; PAGE_SIZE];
            self.file
                .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))?;
            self.file.read(&mut buffer)?;
            e.insert(buffer);
        }
        self.pages
            .get_mut(&page_num)
            .ok_or_else(|| anyhow!("Page not initialized"))
    }

    pub fn flush(&mut self) -> Result<()> {
        for (page_num, page) in self.pages.iter() {
            self.file
                .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))?;
            self.file.write(page)?;
        }
        Ok(())
    }
}
