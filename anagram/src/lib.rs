use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mapped_word = char_count_map(&word.to_lowercase());
    possible_anagrams
        .iter()
        .filter_map(|possible| {
            let lc_possible = possible.to_lowercase();
            if word.to_lowercase() != lc_possible && mapped_word == char_count_map(&lc_possible) {
                Some(*possible)
            } else {
                None
            }
        })
        .collect()
}

fn char_count_map(word: &str) -> HashMap<char, usize> {
    word.chars().fold(HashMap::new(), |mut acc, ch| {
        let counter = acc.entry(ch).or_insert(0);
        *counter += 1;
        acc
    })
}
