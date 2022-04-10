
pub struct KvStore;

impl KvStore {
    pub fn new() -> Self {
        panic!();
    }

    pub fn set(&mut self, key: String, value: String) {
        panic!();
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        panic!();        
    }

    pub fn remove(&mut self, key: String) {
        panic!();        
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
