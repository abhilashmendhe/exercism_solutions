use std::vec;

pub struct Allergies{
    score: u32
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score: score % 256,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        
        let mut f = true;

        match allergen {
            Allergen::Eggs => {
                if self.score & 1 == 0 {
                    f = false;
                }
            },
            Allergen::Peanuts => {
                if self.score & 2 == 0 {
                    f = false;
                }
            },
            Allergen::Shellfish => {
                if self.score & 4 == 0 {
                    f = false;
                }
            },
            Allergen::Strawberries => {
                if self.score & 8 == 0 {
                    f = false;
                }
            },
            Allergen::Tomatoes => {
                if self.score & 16 == 0 {
                    f = false;
                }
            },
            Allergen::Chocolate => {
                if self.score & 32 == 0 {
                    f = false;
                }
            },
            Allergen::Pollen => {
                if self.score & 64 == 0 {
                    f = false;
                }
            },
            Allergen::Cats => {
                if self.score & 128 == 0 {
                    f = false;
                }
            },
            _ => {
                f = false;
            }
        }

        f
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut v: Vec<Allergen> = vec![];
        let mut temp: u32 = 1;
        if temp & self.score> 0 {
            v.push(Allergen::Eggs);
        }
        temp <<= 1;
        if temp & self.score> 0 {
            v.push(Allergen::Peanuts);
        }
        temp <<= 1;
        if temp & self.score> 0 {
            v.push(Allergen::Shellfish);
        }
        temp <<= 1;
        if temp & self.score> 0 {
            v.push(Allergen::Strawberries);
        }
        temp <<= 1;
        if temp & self.score> 0 {
            v.push(Allergen::Tomatoes);
        }
        temp <<= 1;
        if temp & self.score> 0 {
            v.push(Allergen::Chocolate);
        }
        temp <<= 1;
        if temp & self.score> 0 {
            v.push(Allergen::Pollen);
        }
        temp <<= 1;
        if temp & self.score> 0 {
            v.push(Allergen::Cats);
        }
        temp <<= 1;
        v
    }
}
