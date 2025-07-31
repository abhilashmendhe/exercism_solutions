pub fn build_proverb(list: &[&str]) -> String {
    let l_len = list.len();
    if l_len == 0 {
        return format!("{}","");
    }
    if l_len == 1 {
        return format!("And all for the want of a {}.",list[0]);
    }
    let mut nstr: String = String::new();

    for i in 0..(l_len - 1) {
        nstr.push_str(format!("For want of a {} the {} was lost.\n",list[i],list[i+1]).as_str());
    }
    nstr.push_str(format!("And all for the want of a {}.",list[0]).as_str());
    return nstr;
}
