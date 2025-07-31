use std::collections::HashMap;
fn checkNuc(nucleotide: char) -> bool {
    if nucleotide == 'A' || nucleotide == 'C' || nucleotide == 'G' || nucleotide == 'T' {
        return true;
    }
    return false;
}
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {

    if !checkNuc(nucleotide) {
        return Err(nucleotide);
    }

    let mut count: usize = 0;
    for ch in dna.chars() {
        if !checkNuc(ch) {
            return Err(ch);
        }
        if nucleotide == ch {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nuccounts: HashMap<char, usize> = HashMap::new();
    
    match count('A', dna) {
        Ok(cc) => {
            nuccounts.insert('A', cc);
        },
        Err(ch) => return Err(ch)
    };
    match count('C', dna) {
        Ok(cc) => {
            nuccounts.insert('C', cc);
        },
        Err(ch) => return Err(ch)
    };
    match count('G', dna) {
        Ok(cc) => {
            nuccounts.insert('G', cc);
        },
        Err(ch) => return Err(ch)
    };
    match count('T', dna) {
        Ok(cc) => {
            nuccounts.insert('T', cc);
        },
        Err(ch) => return Err(ch)
    };
    

    Ok(nuccounts)
}
