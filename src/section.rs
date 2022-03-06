pub struct Section {
    key: String,
    value: String,
}

impl Section {
    pub fn new(k: String, v: String) -> Self {
        Self { key: k, value: v }
    }

    pub fn serialise(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.key.len() + self.value.len() + 10);

        // FIXME: Casting to u8 is not very safe and won't work with key/values larger than 255 chars
        result.extend_from_slice(&(self.key.len() as u8).to_be_bytes());
        result.extend_from_slice(self.key.as_bytes());
        result.extend_from_slice(&(self.value.len() as u8).to_be_bytes());
        result.extend_from_slice(self.value.as_bytes());

        result
    }
}
