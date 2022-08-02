use anyhow::{ensure, Result};
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

const PAGE_SIZE: usize = 4096;
const MAX_PAGES: usize = 100;

#[derive(Debug)]
struct Page {
    buffer: [u8; PAGE_SIZE],
    default: bool,
}

impl Default for Page {
    fn default() -> Page {
        Page {
            buffer: [0; PAGE_SIZE],
            default: true,
        }
    }
}

pub struct Pager {
    file: File,
    pages: BTreeMap<usize, Page>,
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
            pages: BTreeMap::new(),
        })
    }

    pub fn get_file_size(&self) -> Result<u64> {
        let metadata = self.file.metadata()?;
        Ok(metadata.len())
    }

    pub fn get_page(&mut self, page_num: usize) -> Result<&mut [u8]> {
        ensure!(page_num <= MAX_PAGES, "Page number out of bounds.");
        let page = self.pages.entry(page_num).or_default();
        if page.default {
            self.file
                .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))?;
            self.file.read(&mut page.buffer)?;
            page.default = false;
        }
        Ok(&mut page.buffer)
    }

    pub fn flush(&mut self) -> Result<()> {
        for (page_num, page) in self.pages.iter() {
            self.file
                .seek(SeekFrom::Start((page_num * PAGE_SIZE) as u64))?;
            self.file.write(&page.buffer)?;
        }
        Ok(())
    }
}
