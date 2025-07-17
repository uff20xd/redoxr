mod redoxr;
use redoxr::redoxr::*;
use std::fs;


//use redoxr::redoxr::RedOxR;
//use std::process::{Command, exit};

//TODO: implement auto reflaging for the buildscript
fn main () -> MainResult {

    //automatically self_compiles
    let mut redoxr = Redoxr::new(&[
        "--cfg", "run",
    ]);

    let mut redoxr_lib = RustCrate::new("redoxr", ".")
        .make_lib()
        .set_src(".")
        .set_main("redoxr.rs")
        .set_output_file("libredoxr.rlib")
        .stay();

    let mut main = RustCrate::new("test", ".")
        .make_bin()
        .make_output()
        .depend_on(&mut redoxr_lib)
        .stay();

    _ = redoxr_lib.copy_raw("examples/1_single_crate")?;
    _ = redoxr_lib.copy_raw("examples/2_with_crate_dependencies")?;
    _ = redoxr_lib.copy_raw("examples/3_no_std")?;
    _ = redoxr_lib.copy_raw("examples/4_with_cargo")?;
    compile!(redoxr_lib);
    compile!(main);
    run!(main, "Hello");
    
    let dir = fs::read_dir(".")?;

    for _file in dir {
        //println!("{:?}", file);
        let _command = Cmd::new("ls").output()?;
    }

    Ok(())
}

