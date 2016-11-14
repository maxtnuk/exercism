pub fn primes_up_to(end: u32)-> Vec<u32>{
    (2..end+1).filter(|x|{
        (2..x+1).filter(|n|{
            x%n==0
        }).count()==1}).collect::<Vec<u32>>()
}
