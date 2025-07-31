pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    if upper_bound==1 {
        return v;
    }
    let mut arr:[u64; 1000] = [1;1000];

    for ind in 2..arr.len() {
        if arr[ind] == 0 {
            continue;
        }
        // v.push(ind as u64);
        let mut i = 2;
        while (ind*i) < arr.len() {
            arr[ind*i] = 0;
            i+=1;
        }
    }
    for ind in 2..arr.len() {
        if arr[ind] == 1 {
            if ind <= upper_bound as usize {
            v.push(ind as u64);
            }
        }
    }
    v
}
