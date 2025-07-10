pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn fibonacci(n: u128) -> u128 {
    let mut a = 0;
    let mut b = 0;
    let mut c = 1;

    for _ in 0..n {
        a = b;
        b = c;
        c = a + b;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
