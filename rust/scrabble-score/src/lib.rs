pub fn score(target: &str)-> usize{
    let table=vec![("aeioulnrst",1),
                   ("dg",2),
                   ("bcmp",3),
                   ("fhvwy",4),
                   ("k",5),
                   ("jx",8),
                   ("qz",10)];
    target.chars().fold(0,|acc,n|{
        let mut value_index=0;
        for getc in table.iter(){
            if let Some(_)=getc.0.find(n){
                value_index=acc+getc.1;
                break;
            }else if let Some(_)=getc.0.to_uppercase().find(n){
                value_index=acc+getc.1;
                break;
            }else{
                value_index=acc;
            }
        }
        value_index
    })
}
