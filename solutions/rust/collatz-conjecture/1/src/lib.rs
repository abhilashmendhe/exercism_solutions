pub fn collatz(n: u64) -> Option<u64> {
    if n==0{
        return None;
    } 
    if n==1{
        return Some(0);
    } 

    let mut steps:u64 = 0;

    let mut n = n;
    let three: u64 = 3;
    let one: u64 = 1;
    while n != 1 {
        if n % 2 == 0{
            n /= 2;
        } else {
            n = match n.checked_mul(three) {
                Some(x) => x,
                None => 0,
            };
            if n == 0 {
                return None;
            }
            n = match n.checked_add(one) {
                Some(x) => x,
                None => 0,
            };
            if n == 0 {
                return None;
            }
        }
        steps+=1;
    }
    return Some(steps);
}
