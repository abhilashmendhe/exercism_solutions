use std::collections::HashMap;


pub struct CodonsInfo<'a> {
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    // pairs: Vec<(&'a str, &'a str)>,
    // phantom: std::marker::PhantomData<&'a ()>,
    mps: HashMap<&'a str,&'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.mps.get(&codon){
            Some(val) => return Some(&val),
            None => return None,
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut v: Vec<&'a str> = Vec::new();
        let mut i: usize = 0;
        while i < rna.len() {
            let end:usize = i+3;
            if end > rna.len(){
                return None;
            }
            let val = self.name_for(&rna[i..end]);
            // println!("{:?}, {:?}",val,&rna[i..i+3]);
            if val == None {
                return None;
            }
            if val.unwrap() == "stop codon"{
                break;
            }
            
            v.push(val.unwrap());
            i += 3;
        }
        Some(v)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut hm: HashMap<&'a str, &'a str> = HashMap::new();
    for p in pairs {
        hm.insert(&p.0, &p.1);
    }
    CodonsInfo { mps: hm }
}


// The input data constructor. Returns a list of codon, name pairs.
pub fn make_pairs() -> Vec<(&'static str, &'static str)> {
    let grouped = vec![
        ("isoleucine", vec!["AUU", "AUC", "AUA"]),
        ("valine", vec!["GUU", "GUC", "GUA", "GUG"]),
        ("phenylalanine", vec!["UUU", "UUC"]),
        ("methionine", vec!["AUG"]),
        ("cysteine", vec!["UGU", "UGC"]),
        ("alanine", vec!["GCU", "GCC", "GCA", "GCG"]),
        ("glycine", vec!["GGU", "GGC", "GGA", "GGG"]),
        ("proline", vec!["CCU", "CCC", "CCA", "CCG"]),
        ("threonine", vec!["ACU", "ACC", "ACA", "ACG"]),
        ("serine", vec!["AGU", "AGC"]),
        ("tyrosine", vec!["UAU", "UAC"]),
        ("tryptophan", vec!["UGG"]),
        ("glutamine", vec!["CAA", "CAG"]),
        ("asparagine", vec!["AAU", "AAC"]),
        ("histidine", vec!["CAU", "CAC"]),
        ("glutamic acid", vec!["GAA", "GAG"]),
        ("aspartic acid", vec!["GAU", "GAC"]),
        ("lysine", vec!["AAA", "AAG"]),
        ("arginine", vec!["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"]),
        ("stop codon", vec!["UAA", "UAG", "UGA"]),
    ];
    let mut pairs = Vec::<(&'static str, &'static str)>::new();
    for (name, codons) in grouped.into_iter() {
        for codon in codons {
            pairs.push((codon, name));
        }
    }
    pairs.sort_by(|&(_, a), &(_, b)| a.cmp(b));
    pairs
}