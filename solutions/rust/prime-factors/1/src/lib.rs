pub fn factors(n: u64) -> Vec<u64> {

    let mut v:Vec<u64> = Vec::new();
    let mut i:u64 = 2;
    let mut n = n;
    while n > 1 {
        if n%i == 0 {
            v.push(i);
            n /= i;
        } else {
            i+=1;
        }
    }
    v
}
