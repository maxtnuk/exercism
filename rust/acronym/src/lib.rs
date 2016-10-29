pub fn abbreviate(source: &str)->String{
    let mut choose_result=false;
    let mut another_result=String::new();
    let result=
    source.chars().collect::<Vec<char>>()
    .split(|s| !s.is_alphanumeric()).fold(String::new(),|mut acc,n|{
    if n.len()!=0{
        acc.push(n[0]);
        let mut count =1;
        for i in n.iter().skip(1){
            if i.is_uppercase(){
                acc.push(*i);
                count+=1;
            }
        }
        if n.len()==count {
            choose_result=true;
            another_result=acc.to_uppercase();
        }
    }
    acc
}).to_uppercase();
if choose_result==false{result}else{another_result}
}
