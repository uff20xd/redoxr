mod redoxr;
use redoxr::redoxr::RedOxR;

fn main () -> () {
    let _ = {
        RedOxR::new("build")
            .self_build()
            .set_system_target("x86_64-unknown-linux-gnu")
            .compile();

        RedOxR::new("rustc_tests/main")
            .set_system_target("x86_64-unknown-linux-gnu")
            .compile()
            .run("");

        RedOxR::new("redoxr")
            .generate_crate()
            .set_crate_type("lib")
            .set_system_target("x86_64-unknown-linux-gnu")
            .copy_raw("cargo_tests")
            .compile();
    };
}
