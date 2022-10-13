use std::sync::RwLock;

use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref LUKES_STORE: RwLock<LukesStore> = RwLock::new(LukesStore::new());
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LukesStore {
    pub paths: Vec<String>,
    pub creates: Vec<String>,
    pub removes: Vec<String>,
    pub renames: Vec<String>,
    pub writes: Vec<String>,
}

impl LukesStore {
    pub fn new() -> LukesStore {
        LukesStore {
            paths: vec![],
            creates: vec![],
            removes: vec![],
            renames: vec![],
            writes: vec![],
        }
    }

    pub fn create(&mut self, path_string: String) {
        println!("add path to store {} ", path_string);
        self.paths.push(path_string.clone());
        self.creates.push(path_string);
    }

    pub fn remove(&mut self, path_string: String) {
        println!("remove path to store {} ", path_string);
        self.removes.push(path_string.clone());

        let current_paths: Vec<String> = self.paths.clone();

        // get index of item to remove
        let index: Option<usize> = current_paths
            .into_iter()
            .position(|r| r == path_string.to_string());

        // if the item is found, remove it
        if index.is_some() {
            self.paths.remove(index.unwrap());
        }
    }

    pub fn rename(&mut self, path_source_string: String, path_destination_string: String) {
        println!(
            "remove path to store {} -> {} ",
            path_source_string, path_destination_string
        );

        self.renames.push(path_destination_string.clone());

        let current_paths: Vec<String> = self.paths.clone();
        // get index of item to remove
        let index: Option<usize> = current_paths
            .into_iter()
            .position(|r: String| r == path_source_string);

        // if the item is found, remove it
        if index.is_some() {
            self.paths[index.unwrap()] = path_destination_string;
        }
    }

    pub fn write(&mut self, path_string: String) {
        println!("write path to store {} ", path_string);
        self.writes.push(path_string);
    }
}
