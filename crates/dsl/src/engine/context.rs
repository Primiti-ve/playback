use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    pub vars: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn with(vars: HashMap<String, String>) -> Self {
        Self { vars }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
