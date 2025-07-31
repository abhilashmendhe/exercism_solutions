use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.len() == 0 {
        return 0;
    }
    let mut set: HashSet<u32> = HashSet::new();
    
    for i in 1..limit {

        for fac in factors {
            if *fac == 0_u32 {
                continue;
            }
            if i%fac == 0{
                set.insert(i);
            }
        }
    }
    let mut sum:u32 = 0;
    for val in set {
        sum += val;
    }
    sum
}
