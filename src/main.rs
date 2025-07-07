mod test_lib;
use test_lib::factorial;
//mod libs;
//use libs::crate_test::*;
extern crate redoxr;

use redoxr::redoxr::Mirror;

fn main() {
    let mut test: i32 = 10;
    let mut mirrot_test = Mirror(&mut test);
    test += 1;
    *mirrot_test.borrow_mut() += 5;

    println!("{} ; {}", &test, mirrot_test.borrow());
    mirrot_test.defer();
    println!("{} ; {}",fibbonci(50), factorial(20));
}

fn fibbonci (n: u64) -> u64 {
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
