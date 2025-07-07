mod redoxr;
use redoxr::redoxr::*;


//use redoxr::redoxr::RedOxR;
//use std::process::{Command, exit};
fn main () {

    //automatically self_compiles
    let mut redoxr = Redoxr::new();
    let main_crate = RustCrate::new(&mut redoxr, "test")
        .set_root(".")
        .make_bin() 
        .make_output();
    compile!(main_crate);

}

