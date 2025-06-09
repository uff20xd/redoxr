include!(concat!(env!("OUT_DIR"), "/hello.rs"));
fn main() {
    println!("{}",fibbonci(60));
    message()
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
