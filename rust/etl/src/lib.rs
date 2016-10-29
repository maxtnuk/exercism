use std::collections::BTreeMap;

pub fn transform(source: &BTreeMap<i32,Vec<String>>)->BTreeMap<String,i32>
{
    source.iter().fold(BTreeMap::new(),|mut acc,(value,ref vec)|{
        for x in vec.iter(){
            acc.insert(x.to_lowercase(),*value);
        }
        acc
    })
}
