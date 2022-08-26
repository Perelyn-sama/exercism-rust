// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // unimplemented!()
    let mut map = HashMap::new();
    let mut  v: Vec<&str> = vec![];

    for string in magazine {
        map.insert(string, true);
    }


    for string in note {
        match map.get(string) {
            Some(x) => v.push(string),
            None => println!("You didn't study Web Development")
        }
    }

    note == v
}
