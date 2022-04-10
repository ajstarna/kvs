use std::collections::HashMap;

/// A struture that contains a mapping for String keys to String values
pub struct KvStore {
    mapping: HashMap<String, String>,
}

impl KvStore {
    /// returns a new store with an empty mapping
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }

    /// adds the mapping from key to value
    /// ```
    /// use kvs::KvStore;    
    /// let mut store = KvStore::new();
    /// store.set("adam".to_owned(), "is cool".to_owned());
    /// ```    
    pub fn set(&mut self, key: String, value: String) {
        self.mapping.insert(key, value);
    }

    /// returns the optional value in the mapping for the given key
    /// ```
    /// use kvs::KvStore;    
    /// let mut store = KvStore::new();
    /// store.set("adam".to_owned(), "is cool".to_owned());
    /// assert_eq!(store.get("adam".to_owned()), Some("is cool".to_owned()));    
    /// ```    
    pub fn get(&mut self, key: String) -> Option<String> {
        self.mapping.get(&key).cloned()
    }

    /// removes the the key and its value from the mapping
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("adam".to_owned(), "is cool".to_owned());
    /// assert_eq!(store.get("adam".to_owned()), Some("is cool".to_owned()));        
    /// store.remove("adam".to_owned());
    /// assert_eq!(store.get("adam".to_owned()), None);
    /// ```        
    pub fn remove(&mut self, key: String) {
        self.mapping.remove(&key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
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
