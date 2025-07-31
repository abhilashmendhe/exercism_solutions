use std::vec;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    graVec: Vec<u32>,
    stuVec: Vec<String>
}

impl School {
    pub fn new() -> School {
        School  {
            graVec: vec![],
            stuVec: vec![]
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.graVec.push(grade);
        self.stuVec.push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut nv:Vec<u32> = Vec::new();
        for val in &self.graVec {
            if !nv.contains(val) {
                nv.push(*val);
            }
        }
        nv.sort();
        nv
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut v:Vec<String> = Vec::new();
        for i in 0..self.graVec.len() {
            if &grade == &self.graVec[i] {
                v.push(self.stuVec[i].to_string());
            }
        }
        v.sort();
        v
    }
}
