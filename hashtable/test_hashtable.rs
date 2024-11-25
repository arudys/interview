#![allow(dead_code)]

mod hashtable;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: insufficient arguments {:?}", args);
        return;
    }

    let table_size: usize = args[1].parse().unwrap();

    let mut h: hashtable::HashTable = hashtable::HashTable::new(table_size);

    h.insert("abcde".to_string(), 12);
    h.insert("efgh".to_string(), 33);

    println!("{:?} {:?}", h.get(&"abcde".to_string()), h.get(&"abcdef".to_string()));
    println!("{:?}", h.get_first());
    println!("{:?}", h.get_last());
    h.remove("abcde".to_string());
    println!("{:?} {:?}", h.get(&"abcde".to_string()), h.get(&"abcdef".to_string()));
    println!("{:?}", h.get_first());
    println!("{:?}", h.get_last());
    h.insert("abcde".to_string(), 22);
    println!("{:?} {:?}", h.get(&"abcde".to_string()), h.get(&"abcdef".to_string()));
    println!("{:?}", h.get_first());
    println!("{:?}", h.get_last());
}
