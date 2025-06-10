mod redoxr;
use redoxr::redoxr::RedOxR;

fn main () -> () {
    let _build = RedOxR::new("build")
        .self_build()
        .set_system_target("x86_64-unknown-linux-gnu")
        .compile();

    let _ = RedOxR::new("redoxr")
        .set_src_dir(".")
        .set_crate_type("lib")
        .set_system_target("x86_64-unknown-linux-gnu")
        .copy_raw("crate-test")
        .compile();

    
    let clap = RedOxR::external("external_lib", "https://github.com/clap-rs/clap.git");

    let _main = RedOxR::new("main")
        .set_src_dir("rustc_tests")
        .add_rlib("crate_test")
        .set_system_target("x86_64-unknown-linux-gnu")
        .compile()
        //.add_lib(clap)
        .run("");

}
