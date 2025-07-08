mod redoxr;
use redoxr::redoxr::*;

fn main() -> () {
    let _redoxr = Redoxr::new(&[
        "--cfg", "run",
        "--cfg", "debug"
    ]);



    let mut fibonacci = RustCrate::new("fibonacci", "fibonacci")
        .flags(&["-Copt-level=3"])
        //since the fibonacci/ dir has no own src/ dir you set it to "."
        .set_src(".")
        .set_output_file("libfibonacci.rlib")
        .stay();

    let mut main_crate = RustCrate::new("calculator", ".")
        .flags(&["-Copt-level=3"])
        .make_output()
        .make_bin()
        //Using the incredible Mirror (*mut T Wrapper) Technology 
        //you can pass the fibonacci lib as an uncompiled dependency to multiple crates and compile
        //it later as long as it's compiled when the the crates need it.
        .depend_on(&mut fibonacci)
        .stay();

    compile!(fibonacci);
    compile!(main_crate);
    run!(main_crate);

}
