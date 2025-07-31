use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    
    let mut wordvec:Vec<char> = word.to_lowercase().chars().collect();
    wordvec.sort();
    
    for words in possible_anagrams {
        let mut wordsv:Vec<char> = words.to_lowercase().chars().collect();
        wordsv.sort();
        let mut f = true;
        if wordsv.len() == wordvec.len() &&  word.to_lowercase() != *words.to_lowercase(){
            for ind in 0..wordsv.len() {
                if wordsv[ind] != wordvec[ind] {
                    f = false;
                    break;
                }
            }
        } else {
            f = false;
        }
        if f {
            hs.insert(*words);
        }

    }
    hs
}
