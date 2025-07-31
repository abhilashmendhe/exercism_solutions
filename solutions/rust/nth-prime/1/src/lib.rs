

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut arr:[u32;10001] = [0;10001];
    arr[0] = 2;
    let mut i:u32 = 1;
    let mut num:u32 = 3;
    while i <= n {
        
        let mut f:bool = true;

        for ind in 0..i {
            if num % arr[ind as usize] == 0 {
                f = false;
                break;
            }
        }
        
        
        if !f {
            num+=1;
            continue;
        }
        
        arr[i as usize] = num;
        num += 1;
        i += 1;
    }
    arr[n as usize]

}
