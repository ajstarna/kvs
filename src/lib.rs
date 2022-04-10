use std::collections::{HashMap};

pub struct KvStore {
    mapping: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.mapping.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        self.mapping.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.mapping.remove(&key);
    }
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
