pub fn reverse(input: &str) -> String {
    let mut s: String = String::new();
    for ch in input.chars().rev() {
        s += &ch.to_string();
    }
    s
}
