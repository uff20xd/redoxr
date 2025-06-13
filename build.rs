mod redoxr;
use redoxr::redoxr::Redoxr;

//use redoxr::redoxr::RedOxR;
//use std::process::{Command, exit};
fn main () {
    let _build_script = Redoxr::new("test_bin");
    //let main_crate = RedoxCrate::main(&mut build_script, ".", "src");
}

//let build = Redoxr::build_script();
//if !build.compile() {build.error()};

//let mut redoxr = RedOxR::root("redoxr", ".");
//let _ = redoxr.set_src_dir(".")
//    .set_main_file("redoxr.rs")
//    .set_crate_type("lib");
//    //.set_system_target("x86_64-unknown-linux-gnu");
//    //.copy_raw("crate-test")

//if !build.compile() {build.error()};

//let mut clap = RedOxR::external("clap", "https://github.com/clap-rs/clap.git");
//let mut add = RedOxR::root("crate_test", "crate_test");
//let _ = add.set_crate_builder("cargo");

//let mut main = RedOxR::root("test", ".");
//    let _ = main
//    .set_src_dir("rustc_tests")
//    .add_lib(&mut add)
//    .add_lib(&mut clap);
//    //.add_flag(&["--extern", "clap_complete=rustc_tests/libs/libclap_complete.rlib"])
//    //.add_rlib("crate_test")
//    //.set_system_target("x86_64-unknown-linux-gnu");
//
//if !main.compile() {main.error()};
//if !main.run(&[]) {main.error()};
