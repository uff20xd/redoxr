mod redoxr;
use redoxr::redoxr::*;

const COMMOM_FLAGS: &[&str] = &["-O"];

fn main () -> () {
    
    let mut redoxr = Redoxr::new();

    let mut clap = RedoxCrate::new(&mut redoxr, "clap")
        .flag(COMMON_FLAGS);

    let main_crate = RedoxCrate::main(&mut redoxr, "worseSaltz", "src")
        .dependency(clap)
        .flag(&["-O"]);

    
    if redoxr.compile() {return ();}
}
