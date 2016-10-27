use std::collections::HashMap;

pub fn word_count(target: &str)->HashMap<String, u32>{
    let mut result=HashMap::new();
    let mut letter=String::new();
    let mut is_in: bool=false;
    for src in target.chars(){
        match src.is_alphanumeric(){
            true=>{
                is_in=true;
                letter.push(src);
            },
            false=>{
                if is_in==true{
                    if result.contains_key(&letter.to_lowercase()){
                        *result.get_mut(&letter.to_lowercase()).unwrap()+=1;
                    }else{
                        result.insert(letter.clone().to_lowercase(),1);
                    }
                    letter.clear();
                    is_in=false;
                }else{}
            },
        };
    }
    if letter.len()!=0{
        if result.contains_key(&letter){
            *result.get_mut(&letter).unwrap()+=1;
        }else{
            result.insert(letter.clone(),1);
        }
        letter.clear();
        }
    result
}
