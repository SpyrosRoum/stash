pub struct Section {
    key: String,
    value: String,
}

impl Section {
    pub fn new(k: String, v: String) -> Self {
        Self { key: k, value: v }
    }

    pub fn serialise(&self) -> String {
        let mut result = String::with_capacity(self.key.len() + self.value.len() + 10);

        result.push_str(&self.key.len().to_string());
        result.push('|');
        result.push_str(&self.value.len().to_string());
        result.push('|');
        result.push_str(&self.key);
        result.push('|');
        result.push_str(&self.value);
        result.push('|');

        result
    }
}
