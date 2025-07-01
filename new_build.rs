mod redoxr;
use redoxr::redoxr::*;

const COMMOM_FLAGS: &[&str] = &["-O"];

fn main () -> () {
    
    let mut redoxr = Redoxr::new();

    let mut ron = CargoCrate::new(&mut redoxr, "ron");
    let mut clap = CargoCrate::new(&mut redoxr, "clap");
    let mut serde = CargoCrate::new(&mut redoxr, "serde");

    let main_crate = RedoxCrate::main(&mut redoxr, "test", "src")
        .dependency(oxygencli)
        .dependency(serde);

    if let Some(error) = redoxr.finish() {error.panic()}
}
