pub fn raindrops(x: i32)->String{
    match (x%3,x%5,x%7) {
        (0,five,seven)=>{
            "Pling".to_string()+match (five,seven){
                (0,0)=>&"PlangPlong",
                (0,_)=>&"Plang",
                (_,0)=>&"Plong",
                (_,_)=>&"",
            }
        }
        (_,five,seven)=>{
            match (five,seven){
                (0,0)=>"PlangPlong".to_string(),
                (0,_)=>"Plang".to_string(),
                (_,0)=>"Plong".to_string(),
                (_,_)=>format!("{}",x),
            }
        }
    }
}
