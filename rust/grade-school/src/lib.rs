struct Detail(u32,String);
pub struct School{
    profile: Vec<Detail>,
    total_grade: Vec<u32>,
}
impl School{
    pub fn new()->School{
        School{profile: Vec::new(),
               total_grade: Vec::new()}
    }
    pub fn add (&mut self,grade: u32,name: &str){
        if !self.total_grade.contains(&grade){
            self.total_grade.push(grade);
        }
        self.profile.push(Detail(grade,name.to_string()));
    }
    pub fn grades(&self)->Vec<u32>{
        self.total_grade.clone()
    }
    pub fn grade(&self,grade: u32)->Option<&Vec<String>>{
        Some(
        self.profile.iter().filter(|x| x.0==grade).map(|x| x.1.clone())
            .collect::<Vec<String>>().as_ref())
    }
}
