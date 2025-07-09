extern crate fibonacci;
use fibonacci::*;

trait Animal {
    fn get_legs(&self) -> u32;
}

struct Cat {
    legs: u32
}

impl Animal for Cat  {
    fn get_legs(&self) -> u32 {
        self.legs
    }
}

struct Human {
    legs: u32
}

impl Animal for Human {
    fn get_legs(&self) -> u32 {
        self.legs
    }
}

fn main() -> () {
    let num = 30;
    let fib_of_num = fibonacci(num);

    let human = Human {legs: 2};
    let cat = Cat {legs: 4};


    let test: *const () = &();
    let list_of_animal: [*const dyn Animal; 2] = [&cat, &human];

    for animal in list_of_animal {
        println!("{}", unsafe {(*animal).get_legs()});
    }

    println!("The {}th fibonacci number is {}!", num, fib_of_num)
}
