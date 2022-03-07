use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use anyhow::Result;

use crate::section::Section;

#[derive(Debug)]
pub struct Storage {
    file: File,
}

impl Storage {
    pub fn new(file: File) -> Self {
        Self { file }
    }

    pub fn dump(&mut self) -> Result<()> {
        self.file.rewind()?;

        let mut looking_for_key = true;
        loop {
            let mut len = [0_u8; 1];
            let read = self.file.read(&mut len)?;
            let len = len[0];

            if read == 0 {
                // End of file
                break;
            }

            let mut handle = self.file.try_clone()?.take(len as u64);
            let mut res = String::with_capacity(len as usize);
            handle.read_to_string(&mut res)?;

            if looking_for_key {
                print!("{} = ", res);
            } else {
                println!("{}", res);
            }
            looking_for_key = !looking_for_key;
        }

        Ok(())
    }

    pub fn write(&mut self, section: Section) -> Result<()> {
        // We want to append to the file so move the cursor to the end
        self.file.seek(SeekFrom::End(0))?;

        let section = section.serialise()?;
        self.file.write_all(&section)?;

        Ok(())
    }
}
