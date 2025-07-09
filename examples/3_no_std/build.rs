mod redoxr;
use redoxr::redoxr::*;

fn main() -> MainResult {
    let _redoxr = Redoxr::new(&[
        "--cfg", "run",
        //"--cfg", "debug",
    ]);

    let mut main_crate = RustCrate::new("nostd", ".")
        .flags(&[
            "--edition", "2015",
            //"-Copt-level=z", 
            "-Cpanic=abort",
            "-Clink-arg=-nostartfiles",
            //"--emit=obj"
        ])
        .make_bin()
        .make_output()
        //.set_output_file("nostd.o")
        .stay();

    compile!(main_crate);
    run!(main_crate);
    Ok(())
}
