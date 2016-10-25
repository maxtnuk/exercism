pub fn hamming_distance(target1: &str,target2: &str)->Result<u32,&'static str>{
    if target1.len() != target2.len(){
        Err("not same length")
    }else{
        let mut result: u32=0;
        let mut cmp_char1=target1.chars();
        let mut cmp_char2=target2.chars();
        while let Some(x)=cmp_char1.next(){
            if x!=cmp_char2.next().unwrap(){
                result+=1;
            }else{}
        }
        Ok(result)
    }
}
