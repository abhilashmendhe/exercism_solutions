pub fn series(digits: &str, len: usize) -> Vec<String> {

    let mut v:Vec<String>  = Vec::new();
    let le = digits.len();
    if len > len {
        return v;
    }
    for i in 0..le {
        if le-i < len {
            break;
        }
        // println!("{}",&digits[i..i+len]);
        v.push(digits[i..i+len].to_string());
    }

    v
}
