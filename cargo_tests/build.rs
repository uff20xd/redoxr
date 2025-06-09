mod redoxr;
use redoxr::redoxr_cargo::*;

fn main() {
    let mut truck = Truck::new();

    let _ = TruckFile::new(&truck, 
        "hello.rs").write("
pub fn message () -> () {
    println!(\"this works fine\");
}
");
}
