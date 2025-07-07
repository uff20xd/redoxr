# Adding formless integration
The Idea is to modify main.rs before compilation by adding all the externs to it and cleaning them up afterwards.
This should be optional though.

# Cargo compatibility
i extern the lib using --extern and then add -L release and -L release/deps

'''
bin/
|
|---main
|
|---compiled/
|   |
|   |---liboxygencli.rlib
|
|---src/
    |
    |---main/
    |   |
    |   |---main.rs
    |
    |---oxygencli/
        |
        |---main.rs
'''

/////////////////main.rs////////////////////

sof>>

extern oxygencli;

/////Rest/////


trait RedoxReaction {
    fn compile;
    fn run;
    fn get_outpath;
    fn get_name;
    fn get_outtype;
    fn set_outpath;
    fn dependency;
    fn make_bin;
    fn make_lib;   
    fn make_output;
}

fn main {
    let clap = RustReaction::new("clap", "clap", "src");
    compile!(clap);
    let maincrate = RustReaction::new("some", ".", "src")
        .depend_on(&clap)
        .make_output();
    compile!(maincrate);
    run!(maincrate);
}
