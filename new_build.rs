mod redoxr;
use redoxr::redoxr::*;

const COMMOM_FLAGS: &[&str] = &["-O"];

fn main () -> () {
    let mut redoxr = Redoxr::new()
        .common_flags(COMMON_FLAGS);

    let mut clap = RedoxCrate::new(&mut redoxr, "clap")
        .flag("-O");

    let main_crate = RedoxCrate::main(&mut redoxr, "worseSaltz", "src")
        .dependency(clap)
        .flag(&["-O"]);

    
    if build_script.compile() {return ();}
}
