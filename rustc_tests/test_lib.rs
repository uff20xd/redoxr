pub fn factorial (n: u64) -> u64 {
    let mut l = 1;
    for i in 0..n {
        l = l * i;
    }
    l
}
