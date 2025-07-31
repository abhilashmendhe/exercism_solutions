/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut count: usize = 0;
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    // println!("{}",s1.count());
    for i in 0..s1.len() {
        if &s1[i] != &s2[i] {
            count+=1;
        }
    }
    Some(count)
}
