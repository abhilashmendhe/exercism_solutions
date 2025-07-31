use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut btm:BTreeMap<char, i32> = BTreeMap::new();

    for maps in h {
        // println!("{:?}",maps);
        let value:i32 = *maps.0;
        for ch in maps.1{
            // println!("{}",ch.to_ascii_lowercase());
            btm.insert(ch.to_ascii_lowercase(), value);
        }
    }
    btm
}
