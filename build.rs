mod redoxr;
use redoxr::redoxr::*;


//use redoxr::redoxr::RedOxR;
//use std::process::{Command, exit};
fn main () {

    //automatically self_compiles
    let redoxr = Redoxr::new()
        .debug();
    handle!(redoxr, self_compile);
    handle!(redoxr, setup_env);

//    let redoxr_lib = RustCrate::new(&mut redoxr, "redoxr")
//        .make_lib()
//        .set_root(".")
//        .set_src(".")
//        .set_main("redoxr.rs")
//        .set_output_file("libredoxr.rlib")
//        .stay();
//
//    compile!(redoxr_lib);
    let mut main_crate = RustCrate::new("test", ".")
        .set_root(".")
        .make_bin() 
        .make_output()
        .stay();

    compile!(main_crate);
    run!(main_crate)
}

