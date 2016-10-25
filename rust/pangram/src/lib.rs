pub fn is_pangram(sent: &str)->bool{
    let mut alpabet_table=String::from("abcdefghijklmnopqrstuvwxyz");
    sent.to_lowercase().chars().fold(false ,|_acc,n|{
        for (index,text) in alpabet_table.clone().chars().enumerate(){
            if text==n{alpabet_table.remove(index);}
        }
        if alpabet_table.len()==0{
            true
        }else{
            false
        }
    })
}
