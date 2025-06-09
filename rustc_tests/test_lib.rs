pub fn factorial (n: u64) -> u64 {
    let mut l = 1;
    if n < 2 {
        return 1;
    }
    for i in 1..n {
        l = l * i;
    }
    l
}
