pub fn square_of_sum(source: u64)->u64{
    let mut sum=0;
    for i in 1..source+1{
        sum+=i;
    }
    sum*sum
}
pub fn sum_of_squares(source: u64)->u64{
    let mut result=0;
    for i in 1..source+1{
        result+=i*i;
    }
    result
}
pub fn difference(source: u64)->u64{
    square_of_sum(source)-sum_of_squares(source)
}
