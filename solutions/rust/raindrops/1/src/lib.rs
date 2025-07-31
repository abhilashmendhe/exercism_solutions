pub fn raindrops(n: u32) -> String {
    let mut s: String = String::new();

    if n%3==0 && n%5==0 && n%7==0 {
        s.push_str("PlingPlangPlong");
    } else if n%3==0 && n%5==0 {
        s.push_str("PlingPlang");
    } else if n%3==0 && n%7==0 {
        s.push_str("PlingPlong");
    } else if n%5==0 && n%7==0 {
        s.push_str("PlangPlong");
    } else if n%3==0 {
        s.push_str("Pling");
    } else if n%5==0 {
        s.push_str("Plang");
    } else if n%7==0 {
        s.push_str("Plong");
    } else {
        s = format!("{}",n);
    }

    s
}
