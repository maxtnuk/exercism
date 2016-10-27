use std::ascii::AsciiExt;

pub fn encode(target: &str)->String{
    let alphabet_table="abcdefghijklmnopqrstuvwxyz";
    target.to_lowercase().chars()
        .filter(|x|x.is_alphanumeric() && x.is_ascii()).enumerate()
        .fold(String::new(),|mut acc,(index,x)|{
        acc.push(if let Some(here)=alphabet_table.find(x){
            alphabet_table.chars().rev().nth(here).unwrap()
        }else{x});
        if (index+1)%5==0{
            acc.push(' ');
        }
        acc
    }).trim().to_string()
}
pub fn decode(target: &str)->String{
    let alphabet_table="abcdefghijklmnopqrstuvwxyz";
    target.split_whitespace().flat_map(|s| s.chars())
        .fold(String::new(),|mut acc,x|{
            acc.push(if let Some(here)=alphabet_table.find(x){
                alphabet_table.chars().rev().nth(here).unwrap()
            }else{x});
        acc
    })
}
