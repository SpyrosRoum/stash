use crate::error::{StashError, StashResult};

pub struct Section {
    key: String,
    value: String,
}

impl Section {
    pub fn new(k: String, v: String) -> Self {
        Self { key: k, value: v }
    }

    pub fn serialise(&self) -> StashResult<Vec<u8>> {
        if self.key.len() > u8::MAX as usize {
            return Err(StashError::BadValue("The key is too large".into()));
        }
        if self.value.len() > u8::MAX as usize {
            return Err(StashError::BadValue("The value is too large".into()));
        }

        let mut result = Vec::with_capacity(self.key.len() + self.value.len() + 10);

        result.extend_from_slice(&(self.key.len() as u8).to_be_bytes());
        result.extend_from_slice(self.key.as_bytes());
        result.extend_from_slice(&(self.value.len() as u8).to_be_bytes());
        result.extend_from_slice(self.value.as_bytes());

        Ok(result)
    }
}
