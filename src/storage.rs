use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
};

use anyhow::Result;

use crate::section::Section;

#[derive(Debug)]
pub struct Storage {
    file: File,
    pos: usize,
}

impl Storage {
    pub fn new(file: File) -> Self {
        Self { file, pos: 0 }
    }

    pub fn write(&mut self, section: Section) -> Result<()> {
        // We want to append to the file so move the cursor to the end
        self.file.seek(SeekFrom::End(0))?;

        let section = section.serialise();
        let bytes = section.as_bytes();
        let len = bytes.len();
        self.file.write_all(bytes)?;
        self.pos += len;

        Ok(())
    }
}
