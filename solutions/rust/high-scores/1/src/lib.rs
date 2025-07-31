#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        Self {
            scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }
    pub fn latest(&self) -> Option<u32> {
        let len = self.scores.len();
        if len == 0 {
            return None;
        }
        Some(self.scores[len-1])
    }

    pub fn personal_best(&self) -> Option<u32> {
        
        let len = self.scores.len();
        if len == 0 {
            return None;
        }
        let mut largest = self.scores[0];
        for val in self.scores {
            if *val > largest {
                largest = *val;
            }
        }
        Some(largest)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let len = self.scores.len();
        let mut v:Vec<u32> = Vec::new();
        if len == 0{
            return v;
        }

        v.extend(self.scores);
        v.sort();
        v.reverse();
        if len <= 3 {
            return v;
        }
        let mut i:usize = 1;
        while i < (len-2) {
            v.remove(len - i);
            i+=1;
        }
        v
    }
}
