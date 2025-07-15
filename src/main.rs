mod test_lib;
use test_lib::factorial;
//mod libs;
//use libs::crate_test::*;
extern crate redoxr;
use redoxr::redoxr::Mirror;
use std::env;

fn main() {
    let mut fact = factorial(10);
    let _ = Mirror::new(&mut fact);
    let args: Vec<String> = env::args().collect();

    println!("{} ; {} ; {:?}", fibbonaci(50), factorial(20), args);
}

fn fibbonaci(n: u64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut c: u64 = 1;

    for _ in 0..n {
        a = b;
        b = c;
        c = a + b;
    }
    a
}
