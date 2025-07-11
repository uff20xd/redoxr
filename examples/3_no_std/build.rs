mod redoxr;
use redoxr::redoxr::*;

fn main() -> MainResult {
    let _redoxr = Redoxr::new(&[
        "--cfg", "run",
        //"--cfg", "debug",
    ]);

    let mut main_crate = RustCrate::new("nostd", ".")
        .flags(&[
            "-Copt-level=z",
            "-Cpanic=abort",
            //"-l", "c",
            //"-Clink-arg=-nostartfiles",
            //"-Ctarget-feature=+crt-static",
            "--target=x86_64-unknown-linux-gnu",
            "--target=x86_64-unknown-none",
        ])
        .make_bin()
        .set_output_file("nostd")
        .make_output()
        .stay();

    compile!(main_crate);
    let _ = Cmd::new("gcc").args(&["src/test.c", "-o", "out/test"]).status()?;

    run!(main_crate);
    Ok(())
}
