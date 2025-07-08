extern crate fibonacci;
use fibonacci::*;

fn main() -> () {
    let num = 30;
    let fib_of_num = fibonacci(num);

    println!("The {}th fibonacci number is {}!", num, fib_of_num)
}
