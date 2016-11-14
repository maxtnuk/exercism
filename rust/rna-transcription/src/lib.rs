#[derive(Debug,PartialEq)]
pub struct RibonucleicAcid{
    content: String,
}
#[derive(Debug,PartialEq)]
pub struct DeoxyribonucleicAcid{
    content: String,
}
impl RibonucleicAcid{
    pub fn new(source: &str)->RibonucleicAcid{
        RibonucleicAcid{content: source.to_string()}
    }
}
impl DeoxyribonucleicAcid {
    pub fn new(source: &str)->DeoxyribonucleicAcid{
        DeoxyribonucleicAcid{content: source.to_string()}
    }
    pub fn to_rna(&self)->RibonucleicAcid{
        RibonucleicAcid{content:
        self.content.chars().map(|x|{
            "ATCG".chars().zip("UAGC".chars())
            .filter(|&(d,_r)| d==x).next().unwrap().1
        }).collect::<String>()}
    }
}
