use std::collections::HashSet;


/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    
    let mut hs: HashSet<char> = HashSet::new();
    for ch in sentence.chars(){
        if ch.is_alphabetic() {
            hs.insert(ch.to_ascii_lowercase());
        }
    }
    if hs.len() != 26{
        return false;
    }
    true
}
