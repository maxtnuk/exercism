pub struct Triangle{
    same_side: usize,
}
impl Triangle{
    pub fn build(source: [u32;3])->Result<Triangle,&'static str>{
        match source.iter().all(|&x|x>0){
            true=>  {
                let most_long=source.iter().max().unwrap();
                let sum_side: u32=source.iter().sum();
                if *most_long<sum_side-*most_long{
                    Ok(Triangle{same_side: source.iter().cycle().take(4).cloned()
                                    .collect::<Vec<u32>>().windows(2).filter(|x|x[0]==x[1]).count()})
                }else{
                    Err("we can't make triangle with these")
                }
            }
            false=> Err("wrong length"),
        }
    }
    pub fn is_equilateral(&self)->bool{
        self.same_side==3
    }
    pub fn is_isosceles(&self)->bool{
        self.same_side==1
    }
    pub fn is_scalene(&self)->bool{
        self.same_side==0
    }
}
