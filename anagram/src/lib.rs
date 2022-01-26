use std::collections::HashMap;
use std::collections::HashSet;

fn compute_word_hash(word: &str) -> HashMap<char, u8> {
    word.to_lowercase()
        .chars()
        .fold(HashMap::new(), |mut map, ch| {
            let counter = map.entry(ch).or_insert(0);
            *counter += 1;
            return map;
        })
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_hash = compute_word_hash(word);
    HashSet::from_iter(
        possible_anagrams
            .iter()
            .filter(|anagram| {
                compute_word_hash(anagram) == word_hash
                    && word.to_lowercase() != anagram.to_lowercase()
            })
            .map(|&s| s)
    )
}
