use std::{collections::HashSet};

pub fn check(candidate: &str) -> bool {
    let mut f = true;
    if candidate.len() == 0 {
        return f;
    }   
    
    let candidate = candidate.as_bytes();
    
    let mut hs:HashSet<u8> = HashSet::new();
    let mut c:usize = 0;
    for ch in candidate {
        let mut temp = *ch;
        if temp >= 65 && temp <= 91 {
            temp += 32;
        }

        if temp > 90 && temp < 123 {
            hs.insert(temp);
        } else {
            c+=1;
        }
    }
    if hs.len() != (candidate.len() - c)  {
        f = false;
    }
    f
}
