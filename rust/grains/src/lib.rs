pub fn square(s: u32) -> u64 {
    match s{
        x @ 2 ... 64 => square(x-1)*2,
        1 => 1,
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    square(64)*2-1
}
