use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut words: HashSet<&'a str> = HashSet::new();

    let word = word.to_lowercase();
    let mut sorted_word: Vec<char> = word.chars().collect();
    sorted_word.sort();

    // let mut word : String = sorted_word.into_iter().collect();
    // let word : &str = &word;

    for possible in possible_anagrams {
        let p = possible.to_lowercase();

        if p == word {
            continue;
        }
        let mut p_sorted_word: Vec<char> = p.chars().collect();
        p_sorted_word.sort();

        // let mut p_word : String = p_sorted_word.into_iter().collect();

        if sorted_word == p_sorted_word {
            words.insert(possible);
        }
    }

    words
}
