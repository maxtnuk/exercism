pub fn reply(income: &'static str) -> String{
    {
        if income.is_empty(){"Fine. Be that way!"}
        else{
        let income_char=income.chars().collect::<Vec<char>>();
        match income_char[income.len()-1]{
            '?'=> "Sure.",
            _=>{if income.chars().filter(|&n|n.is_uppercase())
                             .fold(0,|acc,_n|acc+1)>1{
                                 "Whoa, chill out!"
                             }else{
                                 "Whatever."
                             }
                },
            }
        }
    }.to_string()
}
