use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

pub struct Store {
    data: HashMap<String, String>,
    log_file: BufWriter<File>,
}

impl Store {
    // Creates a fresh store — opens/creates the log file for appending,
    // but does NOT read any existing data from it.
    pub fn new(path: &str) -> io::Result<Self> {
        let log_file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Store {
            data: HashMap::new(),
            log_file: BufWriter::new(log_file),
        })
    }
    // line and rebuilds the in-memory HashMap, THEN sets up the store
    // so future writes keep appending to the same file.
    pub fn load(path: &str) -> io::Result<Self> {
        let mut store = Store::new(path)?;

        let read_file = File::open(path)?;
        let reader = BufReader::new(read_file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            match parts.as_slice() {
                ["SET", key, value] => {
                    store.data.insert(key.to_string(), value.to_string());
                }
                ["DELETE", key] => {
                    store.data.remove(*key);
                }
                _ => {}
            }
        }

        Ok(store)
    }

    pub fn set(&mut self, key: String, value: String) -> io::Result<()> {
        writeln!(self.log_file, "SET {} {}", key, value)?;
        self.log_file.flush()?;
        self.data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> io::Result<Option<String>> {
        writeln!(self.log_file, "DELETE {}", key)?;
        self.log_file.flush()?;
        Ok(self.data.remove(key))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn set_and_get() {
        let path = "test_set_and_get.log";
        let _ = fs::remove_file(path);

        let mut store = Store::load(path).unwrap();
        store.set("hello".to_string(), "world".to_string()).unwrap();
        assert_eq!(store.get("hello"), Some(&"world".to_string()));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn delete_removes_key() {
        let path = "test_delete.log";
        let _ = fs::remove_file(path);

        let mut store = Store::load(path).unwrap();
        store.set("hello".to_string(), "world".to_string()).unwrap();
        store.delete("hello").unwrap();
        assert_eq!(store.get("hello"), None);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn reload_replays_log() {
        let path = "test_reload.log";
        let _ = fs::remove_file(path);

        {
            let mut store = Store::load(path).unwrap();
            store
                .set("persisted".to_string(), "yes".to_string())
                .unwrap();
        }

        let store2 = Store::load(path).unwrap();
        assert_eq!(store2.get("persisted"), Some(&"yes".to_string()));

        fs::remove_file(path).unwrap();
    }
}
