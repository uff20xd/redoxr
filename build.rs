mod redoxr;
use redoxr::redoxr::*;


//use redoxr::redoxr::RedOxR;
//use std::process::{Command, exit};
fn main () {

    //automatically self_compiles
    let mut redoxr = Redoxr::new();
//    let redoxr_lib = RustCrate::new(&mut redoxr, "redoxr")
//        .make_lib()
//        .set_root(".")
//        .set_src(".")
//        .set_main("redoxr.rs")
//        .set_output_file("libredoxr.rlib");
//
//    compile!(redoxr_lib);
    let main_crate = RustCrate::new(&mut redoxr, "test")
        .set_root(".")
        .make_bin() 
        .make_output();
    compile!(main_crate);

}

