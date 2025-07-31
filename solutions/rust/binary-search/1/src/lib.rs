pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let mut first:usize = 0;
    let mut last:usize = array.len() -1;
    

    while first<=last {

        let mid:usize = first + (last - first) / 2;
        
        if array[mid] == key {
            return Some(mid);
        }
        if array[mid] < key {
            first = mid + 1;
        } else {
            if mid == 0{
                break;
            }
            last = mid - 1;
        }

    }
    

    None
}
