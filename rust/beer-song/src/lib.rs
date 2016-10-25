fn sentence(bottles: isize)-> String {
    let bottle_string=vec!["no more bottles of beer".to_string(),
               "1 bottle of beer".to_string(),
               bottles.to_string()+" bottles of beer"];
    if bottles<0{
            "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    }else{
    match bottles{
        0 => bottle_string[0].clone(),
        1 => bottle_string[1].clone(),
        _ => bottle_string[2].clone(),
        }
    }
}
pub fn verse(bottles: isize)-> String{
    let mut final_sentence=sentence(bottles);
    if bottles>=0{
        final_sentence.push_str(" on the wall, ");
        final_sentence+=&sentence(bottles);
        final_sentence+=".\n";
        if bottles-1>=0{
            if bottles==1{
            final_sentence+="Take it down and pass it around, ";
        }else{
            final_sentence+="Take one down and pass it around, ";
        }
            final_sentence+=&sentence(bottles-1);
            final_sentence.push_str(" on the wall.\n");
        }else{
            final_sentence+=&sentence(bottles-1);
        }
    }else{}
    let (first_letter,rest)=final_sentence.split_at_mut(1);
    first_letter.to_uppercase()+&rest
}
pub fn sing(end: isize,start: isize)->String{
    let mut song=String::new();
    let mut cnt=end;
    while cnt>=start{
        song+=&verse(cnt);
        if cnt!=start
        {
            song+="\n";
        }
        cnt=cnt-1;
    }
    song
}
