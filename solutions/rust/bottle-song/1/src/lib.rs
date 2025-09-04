pub fn recite(mut start_bottles: u32, mut take_down: u32) -> String {

    let mut output = String::new();

    while take_down > 0 && start_bottles > 0 {
        output.push_str(&create_poem(start_bottles));
        if take_down >= 2{
            output.push('\n');
            output.push('\n');
        }
        start_bottles -= 1;
        take_down -= 1;
    }

    output
}

fn create_poem(start_bottles: u32) -> String {
    let ph1 = if start_bottles >= 2 { 
        format!("{} green bottles hanging on the wall,",num_to_text(start_bottles))
    } else {
        format!("{} green bottle hanging on the wall,",num_to_text(start_bottles))
    };
     
    let ph2 = if start_bottles > 2 {
        format!("There'll be {} green bottles hanging on the wall.",num_to_text(start_bottles-1).to_lowercase())
    } else if start_bottles == 1 {
        format!("There'll be no green bottles hanging on the wall.")
    } else {
        format!("There'll be one green bottle hanging on the wall.")
    };

    format!("{}\n{}\n{}\n{}",&ph1,&ph1,"And if one green bottle should accidentally fall,",ph2)
}

fn num_to_text(start_bottles: u32) -> String {

    match start_bottles {
        10 => "Ten".to_string(),
        9  => "Nine".to_string(),
        8  => "Eight".to_string(),
        7  => "Seven".to_string(),
        6  => "Six".to_string(),
        5  => "Five".to_string(),
        4  => "Four".to_string(),
        3  => "Three".to_string(),
        2  => "Two".to_string(),
        1  => "One".to_string(),
        _ => "".to_string()
    }
}
