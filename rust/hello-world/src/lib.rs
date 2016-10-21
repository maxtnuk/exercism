pub fn hello(name: Option<&str>)-> String{
    let hello: String=match name{
        Some(str)=>{
            "Hello, ".to_string()+str+"!"
        }
        None => "Hello, World!".to_string()
    };
    hello
}
