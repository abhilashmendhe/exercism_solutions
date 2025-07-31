use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    
    let mut v: Vec<&str> = vec![];
    let d_vec:Vec<&str> = diagram.split_whitespace().collect();

    let mut plants: HashMap<char, &str> = HashMap::new();
    plants.insert('V', "violets");
    plants.insert('R', "radishes");
    plants.insert('C', "clover");
    plants.insert('G', "grass");

    let mut stud: HashMap<&str, u8> = HashMap::new();
    stud.insert("Alice",0);
    stud.insert("Bob",2);
    stud.insert("Charlie",4);
    stud.insert("David",6);
    stud.insert("Eve",8);
    stud.insert("Fred",10);
    stud.insert("Ginny",12);
    stud.insert("Harriet",14);
    stud.insert("Ileana",16);
    stud.insert("Joseph",18);
    stud.insert("Kincaid",20);
    stud.insert("Larry",22);

    let first:  &Vec<char> = &d_vec[0].chars().collect();
    let second: &Vec<char> = &d_vec[1].chars().collect();
    
    let id = stud[student];
    let p = &first[id as usize];
    v.push(plants[p]);
    let p = &first[(id+1) as usize];
    v.push(plants[p]);
    let p = &second[id as usize];
    v.push(plants[p]);
    let p = &second[(id+1) as usize];
    v.push(plants[p]);
    v
}
