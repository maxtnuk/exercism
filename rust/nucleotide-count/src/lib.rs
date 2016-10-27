use std::collections::HashMap;

pub fn nucleotide_counts(target: &str)->HashMap<char,usize>{
    let mut result=HashMap::new();
    result.insert('A', count('A',target));
    result.insert('C', count('C',target));
    result.insert('T', count('T',target));
    result.insert('G', count('G',target));
    result
}
pub fn count(target :char,source: &str)->usize{
    source.chars().filter(|x| *x==target).count()
}
