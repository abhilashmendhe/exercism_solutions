/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.trim();
    if code.len() <= 1 {
        return false;
    }
    let mut f = true;
    let mut switch = false;
    let mut sum = 0;
    println!("{}",code);
    for ch in code.chars().rev() {
        if ch == ' ' { 
            continue;
        }
        let val = ch.to_digit(10);
        
        match val {
            Some(val) => {
                if switch{
                    let mut temp = val * 2;
                    if temp > 9 {
                        temp -= 9;
                    }
                    sum += temp;
                } else {
                    sum += val;
                }
            } 
            None => {
                f = false;
                break;
            }
        }
        switch = !switch;
    }
    if sum % 10 != 0{
        f = false;
    }
    f
}
