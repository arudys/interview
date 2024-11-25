#![allow(dead_code)]

use std::hash::{Hash, Hasher, DefaultHasher};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct HashTable {
    table: Vec<Option<HashNode>>,
    mod_history: VecDeque<usize>,
    len: usize,
}

#[derive(Clone, Debug)]
pub struct HashNode {
    pub key: String,
    pub value: i64,
    live: bool,
}

impl HashNode {
    pub fn new(key: String, value: i64 ) -> HashNode {
        HashNode { key: key, value: value, live: true }
    }
}

impl HashTable {
    pub fn new(table_size: usize) -> HashTable {
        println!("Init hashtable, size {}", table_size);
        HashTable {
            table: vec![None; table_size],
            mod_history: VecDeque::new(),
            len: 0,
        }
    }

    // Gets the default hash index of the key.
    fn get_hash_index(&self, key: &String) -> usize {
        let mut hasher = DefaultHasher::new();

        key.hash(&mut hasher);
        hasher.finish() as usize % self.table.len()
    }

    // Gets the node index of a specified key:
    // If the key is in the table and live, return that index.
    // Otherwise, return the first free index (a free index is either
    // None or Some but not live).
    //
    // Returns None if the key is not in the table and there is no
    // free slot in the table.
    fn get_node_index(&self, key: &String) -> Option<usize> {
        let orig_hash_index = self.get_hash_index(key);
        let mut hash_index = orig_hash_index;
        let mut has_free = false;
        let mut first_free: usize = 0;

        loop {
            let nod = &self.table[hash_index];
            match nod {
                // hash nodes only ever go None ==> Some, never Some
                // ==> None. This means once we encounter a None, we
                // know we can stop looping.
                None => return Some(if has_free { first_free } else { hash_index }),
                Some(n) => if n.key == *key {
                    if n.live || !has_free {
                        return Some(hash_index);
                    } else {
                        return Some(first_free);
                    }
                } else if !n.live && !has_free {
                    has_free = true;
                    first_free = hash_index;
                },
            }

            hash_index += 1;
            hash_index = hash_index % self.table.len();

            if hash_index == orig_hash_index {
                // We've looped and found no matching or empty slots. See if
                // there's a slot we can reclaim.
                if has_free {
                    return Some(first_free);
                }
                return None;
            }
        }
    }

    pub fn insert(&mut self, key: String, value: i64) {
        let hash_index_opt = self.get_node_index(&key);
        if hash_index_opt.is_none() {
            println!("Failed to get node index, table is full");
            return;
        }
        let hash_index = hash_index_opt.unwrap();
        match self.table[hash_index].as_mut() {
            None => {
                self.table[hash_index] = Some(HashNode::new(key, value));
                self.len += 1;
            }
            Some(n) => {
                if !n.live {
                    self.len += 1;
                }

                // O(n) operation. Making this O(1) would require a custom
                // double-linked list, which is Hard(tm) in Rust.
                self.mod_history.retain(|&x| x != hash_index);
                n.key = key;
                n.value = value;
                n.live = true;
            }
        }

        self.mod_history.push_back(hash_index);
    }

    // Gets the node for a given key, or None if the key is not in the table.
    fn get_node(&self, key: &String) -> Option<&HashNode> {
        let hash_index = self.get_node_index(key)?;

        if self.table[hash_index].is_some() && !self.table[hash_index].as_ref().unwrap().live {
            return None;
        }

        self.table[hash_index].as_ref()
    }

    // Gets the value for a given key, or None if the key is not in the table.
    pub fn get(&self, key: &String) -> Option<i64> {
        return Some(self.get_node(&key)?.value);
    }

    // Remove a key from the table.
    pub fn remove(&mut self, key: String) {
        let hash_index_opt = self.get_node_index(&key);
        if hash_index_opt.is_none() {
            println!("Failed to get node index, table is full");
            return;
        }
        let hash_index = hash_index_opt.unwrap();

        match self.table[hash_index].as_mut() {
            None => return,
            Some(n) => {
                // If the node returned by get_node_index() is live,
                // it always points to the specified key.
                if n.live {
                    n.live = false;
                    self.len -= 1;
                }
            }
        }
    }

    pub fn get_first(&mut self) -> Option<&HashNode> {
        let mut first_index = self.mod_history.front();
        if first_index.is_none() {
            return None;
        }

        loop {
            let first_index_val = *first_index.unwrap();

            if self.table[first_index_val].as_ref().unwrap().live {
                return self.table[first_index_val].as_ref();
            }

            self.mod_history.pop_front();
            first_index = self.mod_history.front();
            if first_index.is_none() {
                return None;
            }
        }
    }

    pub fn get_last(&mut self) -> Option<&HashNode> {
        let mut last_index = self.mod_history.back();

        if last_index.is_none() {
            return None;
        }

        loop {
            let last_index_val = *last_index.unwrap();

            if self.table[last_index_val].as_ref().unwrap().live {
                return self.table[last_index_val].as_ref();
            }

            self.mod_history.pop_back();
            last_index = self.mod_history.back();
            if last_index.is_none() {
                return None;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
