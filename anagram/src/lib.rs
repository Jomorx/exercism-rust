use std::{collections::HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    let lower_word = word.to_lowercase();
    let sorted_word = sort_word(&lower_word);

    for &anagram in possible_anagrams  {
        let lower_anagram = anagram.to_lowercase();

        if lower_word != lower_anagram 
            && sorted_word == sort_word(&lower_anagram)    {
            set.insert(anagram);
        }
    }

    set
}

fn sort_word (word:&str)-> String {
    let mut word:Vec<char> = word.chars().collect();
    word.sort();
    word.into_iter().collect()
} 