mod redoxr;
use redoxr::redoxr::*;

fn main() -> MainResult {
    let _redoxr = Redoxr::new(&[
        "--cfg", "quiet",
        //"--cfg", "run",
        //"--cfg", "debug",
    ]);

    let mut main_crate = RustCrate::new("nostd", ".")
        .flags(&[
            "-Cpanic=abort",
            "--edition", "2024",
            "--target", "x86_64-unknown-none",
        ])
        .make_bin()
        //.set_output_file("nostd")
        .make_output()
        .stay();

    compile!(main_crate);
    //let _ = Cmd::new("gcc").args(&["src/test.c", "-o", "out/test"]).status()?;

    run!(main_crate);
    Ok(())
}
