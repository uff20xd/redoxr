mod redoxr;
use redoxr::redoxr::*;

fn main() -> MainResult {
    let redoxr = Redoxr::new(&[
        "--cfg", "run",
    ]);

    let mut cargo_crate =RustCrate::from_cargo("fibonacci", "fibonacci")
        .stay();

    let mut main_crate = RustCrate::new("with_cargo", ".")
        .set_src(".")
        .make_bin()
        .make_output()
        .stay();

    let _ = cargo_crate.compile_cargo()?;
    compile!(main_crate);
    Ok(())
}
