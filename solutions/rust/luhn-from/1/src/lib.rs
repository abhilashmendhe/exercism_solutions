
pub struct Luhn{
    num: String
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.num.len() <= 1{
            return false;
        }
        let mut switch: bool = false;
        let mut sum: u32 = 0;
        for ch in self.num.chars().rev(){
            if ch == ' '{
                continue;
            }
            match ch.to_digit(10){
                Some(mut n) => {
                    if switch {
                        n = n * 2;
                        if n > 9 {
                            n -= 9;
                        }
                    }
                    sum += n;
                },
                None => {
                    return false;
                }
            }
            switch = !switch;
        }
        if sum % 10 == 0 {
            return true; 
        } else {
            return false;
        }
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn{
            num:input.to_string()
        }
    }
}
