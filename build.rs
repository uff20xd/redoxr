mod redoxr;
use redoxr::redoxr::*;


//use redoxr::redoxr::RedOxR;
//use std::process::{Command, exit};

//TODO: implement auto reflaging for the buildscript
fn main () {

    //automatically self_compiles
    let mut redoxr = Redoxr::new(&[
        "--cfg", "manual",
    ]);
    handle!(redoxr, self_compile);
    handle!(redoxr, setup_env);

    let mut redoxr_lib = RustCrate::new("redoxr", ".")
        .make_lib()
        .set_src(".")
        .set_main("redoxr.rs")
        .set_output_file("libredoxr.rlib")
        .stay();

    if let Some(error) = redoxr_lib.copy_raw("examples/1_single_crate") {panic!("{}", error)}
    if let Some(error) = redoxr_lib.copy_raw("examples/2_with_crate_dependencies") {panic!("{}", error)}
    if let Some(error) = redoxr_lib.copy_raw("examples/3_no_std") {panic!("{}", error)}
    compile!(redoxr_lib);
}

