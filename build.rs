mod redoxr;
use redoxr::redoxr::RedOxR;

fn main () {
    let _build = RedOxR::build_script()
        .compile();

    let _ = RedOxR::root("redoxr", ".")
        .set_src_dir(".")
        .set_main_file("redoxr.rs")
        .set_crate_type("lib")
        .set_system_target("x86_64-unknown-linux-gnu")
        //.copy_raw("crate-test")
        .compile();

    let clap = RedOxR::external("external_lib", "https://github.com/clap-rs/clap.git");

    let main = RedOxR::root("test", ".")
        .set_src_dir("rustc_tests")
        .add_rlib("crate_test")
        .set_system_target("x86_64-unknown-linux-gnu");

    if !main.compile() {main.error()};
    if !main.run(&[]) {main.error()};
}
