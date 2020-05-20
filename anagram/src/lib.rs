use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    let mapped_word = char_count_map(word);
    for possible in possible_anagrams {
        if word.to_lowercase() != possible.to_lowercase() &&
            mapped_word == char_count_map(possible) {
            set.insert(*possible);
        }
    }
    set
}

fn char_count_map(word: &str) -> HashMap<char, usize> {
    word.chars()
        .fold(HashMap::new(), |mut acc, ch| {
            for c in ch.to_lowercase() {
                let counter = acc.entry(c).or_insert(0);
                *counter += 1;
            }
            acc})
}
