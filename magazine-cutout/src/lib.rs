// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

fn make_map<'a>(words: &'a [&str]) -> HashMap<&'a str, i32> {
    let mut word_map = HashMap::new();
    for word in words {
        *word_map.entry(*word).or_insert(0) += 1;
    }
    return word_map;
}
pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_map = make_map(magazine);
    let note_map = make_map(note);
    note_map.into_iter().all(|(k, v)| *magazine_map.get(k).unwrap_or(&0) >= v)
}
