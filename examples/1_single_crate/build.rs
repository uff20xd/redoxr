mod redoxr; 
use redoxr::redoxr::*;

fn main() -> () {
    //Defines the build_script needed for configs to apply.
    //This can be skipped if you dont want bootstrapping
    //or manual rebuilding.
    //The --cfg run option is passed so the compiled file
    //will also be run after being compiled.
    let _redoxr = Redoxr::new(&[
        "--cfg", "run",
    ]);

    //This is the variable that contains the main crate.
    let mut main_crate = RustCrate::new("test", ".")
        //Sets the file as an executable.
        .make_bin() 
        //Puts the file in the out/ dir instead of the out/deps/ dir
        .make_output()
        //Ends the statement (needed cause I suck at coding)
        .stay();

    compile!(main_crate);
    run!(main_crate);
}
