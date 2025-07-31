#[derive(Debug)]
pub struct ChessPosition{
    x: i32,
    y: i32
}

#[derive(Debug)]
pub struct Queen{
    pos: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (rank >= 0 && rank <= 7) && (file >= 0 && file <= 7) {
            return Some(ChessPosition{
                x:rank,
                y:file
            });
        }
        None
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos:position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        
        if self.pos.x == other.pos.x ||  self.pos.y == other.pos.y{
            return true;
        }  
        let mut v1 = self.pos.x - other.pos.x;
        if v1 < 0 {
            v1 *= -1;
        }
        let mut v2 = self.pos.y - other.pos.y;
        if v2 < 0 {
            v2 *= -1;
        }
        if v1 == v2 {
            true
        } else {
            false
        }
    }
}
