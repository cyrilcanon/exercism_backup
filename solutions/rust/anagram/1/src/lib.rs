use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams:  &[&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&'a str> = HashSet::new();
    let mut word_sorted: Vec<char> = word.to_lowercase().chars().collect();
    word_sorted.sort();
    for word_i in possible_anagrams.iter(){
        let mut word_i_sorted: Vec<char> = word_i.to_lowercase().chars().collect();
        word_i_sorted.sort();
        if word_sorted == word_i_sorted{
            if word.to_lowercase() != *word_i.to_lowercase() {
                res.insert(*word_i);
            }
        }
    }
    res
}
