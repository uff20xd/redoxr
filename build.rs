extern crate redoxr;
use redoxr::RedOxR;

fn main () -> () {
    let _ = {
        RedOxR::new("build")
            .self_build()
            .set_system_target("x86_64-unknown-linux-gnu")
            .compile();

        RedOxR::new("redoxr")
            .generate_crate()
            .set_crate_type("lib")
            .set_system_target("x86_64-unknown-linux-gnu")
            .compile();
    };
}
