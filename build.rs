mod redoxr;
use redoxr::redoxr::*;


//use redoxr::redoxr::RedOxR;
//use std::process::{Command, exit};

//TODO: implement auto reflaging for the buildscript
fn main () {

    //automatically self_compiles
    let mut redoxr = Redoxr::new(&[]);
    handle!(redoxr, self_compile);
    handle!(redoxr, setup_env);

    let mut redoxr_lib = RustCrate::new("redoxr", ".")
        .make_lib()
        .set_src(".")
        .set_main("redoxr.rs")
        .set_output_file("libredoxr.rlib")
        .stay();

    compile!(redoxr_lib);
    let mut main_crate = RustCrate::new("test", ".")
        .flags(&["--edition", "2024"])
        .set_root(".")
        .make_bin() 
        .make_output()
        .depend_on(&mut redoxr_lib)
        .stay();

    compile!(main_crate);
    run!(main_crate)
}

