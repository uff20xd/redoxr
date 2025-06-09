mod redoxr;
use redoxr::redoxr::RedOxR;

fn main () -> () {
    let build = RedOxR::new("build")
        .self_build()
        .set_system_target("x86_64-unknown-linux-gnu")
        .compile();

    let _ = RedOxR::new("redoxr")
        .generate_crate()
        .set_crate_type("lib")
        .set_system_target("x86_64-unknown-linux-gnu")
        .copy_raw("cargo_tests")
        .compile();

    
    let external_lib = RedOxR::external("external_lib", "https://github.com/clap-rs/clap.git")
        .make_mod("cargo");


    let main = RedOxR::new("rustc_tests/main")
        .set_system_target("x86_64-unknown-linux-gnu")
        .compile()
        //.add_lib(external_lib)
        .run("");

}
