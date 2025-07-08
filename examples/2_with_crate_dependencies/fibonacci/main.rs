pub fn fibonacci(n: u128) -> u128 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1
    for i in 0..n {
        a = b;
        b = c;
        c = a + b;
    }
    a
}
