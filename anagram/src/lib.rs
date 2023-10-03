use std::{collections::HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    for &anagram in possible_anagrams  {
        if word.to_lowercase() != anagram.to_lowercase() 
            && sort_word(word) == sort_word(anagram)    {
            set.insert(anagram);
        }
    }

    set
}

fn sort_word (word:&str)-> String {
    let mut word:Vec<char> = word.to_lowercase().chars().collect();
    word.sort();
    word.into_iter().collect()
}