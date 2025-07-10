mod redoxr;
use redoxr::redoxr::*;

fn main() -> MainResult {
    let _redoxr = Redoxr::new(&[
        "--cfg", "run",
        //"--cfg", "debug",
    ]);

    let mut main_crate = RustCrate::new("nostd", ".")
        .flags(&[
            "--edition", "2024",
            //"-Copt-level=z",
            "-Cpanic=abort",
            "-l", "c",
            //"-Clink-arg=-nostartfiles",
            //"-Ctarget-feature=+crt-static",
            "--target=x86_64-unknown-none",
            //"--emit=obj"
        ])
        .make_bin()
        .make_output()
        //.set_output_file("nostd.o")
        .stay();

    let _ = Cmd::new("gcc").args(&["src/test.c", "-o", "out/test"]).spawn()?.wait()?;
    compile!(main_crate);
    run!(main_crate);
    Ok(())
}
