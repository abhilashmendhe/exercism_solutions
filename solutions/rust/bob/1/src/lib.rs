pub fn reply(message: &str) -> &str {

    let msg = message.trim();
   
    if msg.len() <= 0 {
        return "Fine. Be that way!";
    }

    let mut all_caps = true;
    let mut non_letter_count = 0;

    let msg_bytes = msg.bytes().collect::<Vec<u8>>();
    for ch in &msg_bytes {
        if ch.is_ascii_alphabetic() {
            
            if *ch < 65 || *ch > 90 {
                all_caps = false;
                break;
            }
        } else {
            non_letter_count += 1;
        }
    }

    if non_letter_count == msg_bytes.len() {
        let ch =  *(&msg_bytes[msg_bytes.len()-1]) as char;
        if ch == '?' {
            return "Sure.";
        } else {
            return "Whatever.";
        }
    };
    if all_caps {

        let ch =  *(&msg_bytes[msg_bytes.len()-1]) as char;
        
        if ch == '?' {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }

    } else {
        let ch =  *(&msg_bytes[msg_bytes.len()-1]) as char;
        if ch == '?' {
            return "Sure.";
        } else {
            return "Whatever.";
        }
    }
    
}
