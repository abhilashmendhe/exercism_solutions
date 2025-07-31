pub fn egg_count(display_value: u32) -> usize {
    let mut count:usize = 0;
    let mut i:u32 = 1;
    while i < 2147483648 {
        if i&display_value != 0{
            count += 1;
        }
        println!("{}, {}",i,i&display_value);
        i <<= 1;
    }
    count
}
