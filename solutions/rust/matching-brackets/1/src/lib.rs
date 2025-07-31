pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in string.chars() {
        // println!("{}",ch);
        if ch=='(' || ch=='[' || ch=='{' {
            stack.push(ch);
        } else {
            if ch==')' && stack.ends_with(&['(']) {
                stack.pop();
            } else if ch=='}' && stack.ends_with(&['{']) {
                stack.pop();
            } else if ch==']' && stack.ends_with(&['[']) {
                stack.pop();
            } else if ch==')' || ch==']' || ch=='}' {
                stack.push(ch);
            }
        }
        
    }
    if stack.len() == 0 {
        return true;
    }
    false
}
