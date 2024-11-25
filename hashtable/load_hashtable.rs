#![allow(dead_code)]
#![allow(non_snake_case)]

mod hashtable;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::str::Chars;

// From a stream of chars, extract the next "word", defined as a contiguous
// range of alphabetic chars. Make lowercase for normalization.
fn NextWord(chars: &mut Chars) -> Option<String> {
    let mut ret: String = String::new();

    loop {
        match chars.next() {
            Some(c) => {
                if c.is_alphabetic() {
                    if c.is_uppercase() {
                        for cc in c.to_lowercase() {
                            ret.push(cc);
                        }
                    } else {
                        ret.push(c);
                    }
                } else {
                    if !ret.is_empty() {
                        return Some(ret);
                    }
                }
            }
            None => {
                if ret.is_empty() {
                    return None;
                }
                return Some(ret);
            }
        }
    }
}

// Split a line into words and insert in the hashtable.
fn InsertLine(h: &mut hashtable::HashTable, line: &String) {
    let mut chars = line.chars();

    loop {
        match NextWord(&mut chars) {
            Some(w) =>
                match h.get(&w) {
                    Some(count) => h.insert(w, count + 1),
                    None => h.insert(w, 1),
                },
            None => {
                println!("First: {:?}", h.get_first());
                println!("Last: {:?}", h.get_last());
                return;
            }
        }
    }
}

fn LoadHashtable(h: &mut hashtable::HashTable, filename: &String) -> std::io::Result<()> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    loop {
        let mut line = String::new();
        let l = reader.read_line(&mut line)?;

        // EOF
        if l == 0 {
            break;
        }

        InsertLine(h, &line);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Error: insufficient arguments {:?}", args);
        println!("Expect: load_hashtable <hashtable size> <input file>");
        return Ok(());
    }

    let table_size: usize = args[1].parse().unwrap();
    let mut h: hashtable::HashTable = hashtable::HashTable::new(table_size);

    LoadHashtable(&mut h, &args[2])?;
    println!("Len: {}", h.len());

    Ok(())
}
