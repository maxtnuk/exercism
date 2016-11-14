pub struct Roman{
    content: Vec<usize>
}
impl Roman{
    pub fn from(source: usize)->Roman{
        let mut convert=Vec::new();
        let mut current=source;
        loop{
            convert.push(current%10);
            if current<10{
                break;
            }
            current/=10;
        }
        Roman{content: convert}
    }
    pub fn to_string(&self)->String{
        let table="IVXLCDM";
        self.content.iter().enumerate()
            .fold(Vec::new(),|mut acc,(index,&value)|{
                let mut outcome=String::new();
                if value<5{
                    match value%5{
                        x @ 0...3=> {
                            for _ in 0..x{
                                outcome.push(
                                    table.chars().nth(2*index).unwrap()
                                );
                            }},
                            4=>{
                                outcome.push(
                                    table.chars().nth(2*index).unwrap()
                                );
                                outcome.push(
                                    table.chars().nth(2*index+1).unwrap()
                                );
                            },
                            _ => {},
                        }
                }else{
                    match value%5{
                        x @ 0...3=> {
                            outcome.push(table.chars().nth(2*index+1).unwrap());
                            for _ in 0..x{
                                outcome.push(
                                    table.chars().nth(2*index).unwrap()
                                );
                            }},
                            4=>{
                                outcome.push(
                                    table.chars().nth(2*index).unwrap()
                                );
                                outcome.push(
                                    table.chars().nth(2*index+2).unwrap()
                                );
                            },
                            _ => {},
                        }
                    }
                acc.push(outcome);
                acc
            }).iter().rev().cloned().collect::<String>()
    }
}
