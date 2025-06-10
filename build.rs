mod redoxr;
use redoxr::redoxr::RedOxR;
//use std::process::{Command, exit};

fn main () {

    let build = RedOxR::build_script();
    if !build.compile() {build.error()};

    let _redoxr = RedOxR::root("redoxr", ".")
        .set_src_dir(".")
        .set_main_file("redoxr.rs")
        .set_crate_type("lib")
        .set_system_target("x86_64-unknown-linux-gnu");
        //.copy_raw("crate-test")

    if !build.compile() {build.error()};

    let clap = RedOxR::external("clap", "https://github.com/clap-rs/clap.git");
    let add = RedOxR::root("crate_test", "crate_test").set_crate_builder("cargo");

    let main = RedOxR::root("test", ".")
        .set_src_dir("rustc_tests")
        .add_lib(add)
        .add_lib(clap)
        //.add_rlib("crate_test")
        .set_system_target("x86_64-unknown-linux-gnu");
    
    if !main.compile() {main.error()};
    if !main.run(&[]) {main.error()};
}
