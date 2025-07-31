pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let mut temp_num: u64 = num as u64;
    let len: u32 = temp_num.ilog10() + 1;
    let mut sum: u64 = 0;
    while temp_num > 0 {
        let modu: u64 = temp_num % 10;
        sum += modu.pow(len);
        temp_num /= 10;
    }
    if sum == num as u64 {
        return true;
    } else {
        return false;
    }
    
}