pub fn private_key(p: u64) -> u64 {
    2
}
// A = gᵃ mod p
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let mut a = a;
    let mut g = g;
    let mut res:u64 = 1;

    while a > 0 {
        if a % 2 == 1 {
            res = (res * g)%p;
        }
        a = a >> 1;

        g = (g * g)%p;
    }
    res % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
   let mut a = a;
    let mut g = b_pub;
    let mut res:u64 = 1;

    while a > 0 {
        if a % 2 == 1 {
            res = (res * g)%p;
        }
        a = a >> 1;

        g = (g * g)%p;
    }
    res % p
}
