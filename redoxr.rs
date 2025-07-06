//====================================================================
//Redoxr is a open-source build_scripter inspired by nob.h (https://github.com/tsoding/nob.h) and
//Cargo (https://github.com/rust-lang/cargo).
//
//Its not made to be used in actual projects and does not fit many use cases.
//On the contrary it does help with actually thinking about what you are building and what libs you
//really need.
//There is also (planned) cross-compatitibility with Cargo both ways. 
//Cargo to Redox will be added first and Redox to Cargo will be added using Truck (see at the
//bottom).
//All outputs will be put in the bin/ (and bin/deps/) directory, which sanitizes the
//environment and adds reproducability.
//
//This is just the start and in the end I want to be able to compile anything using Redoxr and also
//be able to easily extend it to any other language using "intelligent trait usage".
//
//Basic Usage:
//####build.rs####
//mod redoxr;
//use redoxr::redoxr::*;
//
//fn main() -> () {
//    let mut redoxr = Redoxr::new();
//    let mut main_crate = RustCrate::main(&mut redoxr, "some_crate");
//    if let Some(error) = main_crate.compile() {error.panic()};
//}
//################
//
//With dependencies:
//####build.rs####
//mod redoxr;
//use redoxr::redoxr::*;
//
//fn main() -> () {
//    let mut redoxr = Redoxr::new();
//    let mut dependency = RustCrate::from_cargo(&mut redoxr, "clap");
//
//    if let Some(error) = 
//
//    let mut main_crate = RustCrate::main(&mut redoxr, "some_crate").
//        .dependency(dependecy.clone());
//
//}
//################
//
//====================================================================
#![allow(dead_code)]

#[cfg(target_os = "linux")]
pub const PATH_SEPERATOR: &'static str = r"/";

#[cfg(target_os = "windows")]
pub const PATH_SEPERATOR: &'static str = r"\";

pub mod redoxr {
    #[cfg(target_os = "linux")]
    pub const PATH_SEPERATOR: &'static str = r"/";
    
    #[cfg(target_os = "windows")]
    pub const PATH_SEPERATOR: &'static str = r"\";

    ///Marking unfinished fields needed for later.
    struct EmptyField;

    use std::{
        process::{
            Command, //Child,
        }, 
    };
 
    #[derive(Debug, Clone)]
    enum CrateBuilder {
        SingleFile,
        Cargo,
        RedOxR,
        PreBuilt,
        NoneAtAll,
    }

    #[derive(Debug)]
    pub enum RedoxError {
        Error
    }
    impl RedoxError {
        pub fn panic (&self) -> () {
            dbg!(self);
            panic!("Error encountered at ...");
        }
    }

    enum CrateType {
        Lib,
        Bin,
        CargoLib,
        CargoBin,
        RedoxManagedBin,
        RedoxManagedLib,
        Empty,
    }

    pub trait RedoxrCompatible {
        fn empty() -> Self;
        fn compile(&self) -> Option<RedoxError>;
        fn get_outputs(&self) -> String;
    }
    // Implement Concept for better builds
    
    ///A Struct that defines a Rust Crate managed by any build system
    pub struct RustCrate {
        name: String,
        root: String,
        src_dir: String,

        deps: Vec<u32>,
        crate_type: CrateType,

        flags: Vec<String>,

        refrence_counter: u64,


        //currently unused
        id: u64,
        external: Option<String>,
    }

    #[macro_export]
    ///A macro so you don't have to type out the entire if-let-statement.
    ///Takes the crate to compile as input.
    macro_rules! compile {
        ($comp_file:ident) => {
            if let Some(error) = ($comp_file).compile() {error.panic()}
        }
    }

    /// Struct that defines a crate for as the main file or a dependency
    impl RustCrate {
        pub fn empty() -> Self {
            let call = Self {
                name: "".to_owned(),
                root: "".to_owned(),
                src_dir: "".to_owned(),

                deps: Vec::new(),
                crate_type: CrateType::Empty,

                flags: Vec::new(),

                id: 0,
                refrence_counter: 0,

                external: None,
            };
            call
        }

        pub fn main<'a>(script: &'a mut Redoxr, name:&str) -> &'a mut Self {
            let call = Self {
                name: name.to_owned(),
                root: ".".to_owned(),
                src_dir: "src".to_owned(),

                deps: Vec::new(),
                crate_type: CrateType::Lib,

                flags: Vec::new(),

                id: 0,
                refrence_counter: 0,

                external: None,
            };
            script.set_main(call)
        }

        pub fn new<'a>(script: &'a mut Redoxr, name: &str) -> &'a mut Self {
            let call = Self {
                name: name.to_owned(),
                root: name.to_owned(),
                src_dir: "src".to_owned(),

                deps: Vec::new(),
                crate_type: CrateType::Lib,

                flags: Vec::new(),

                id: 0,
                refrence_counter: 0,

                external: None,
            };
            script.add_crate(call)
        }

        pub fn from_cargo<'a>(_script: &'a mut Redoxr, _name: &str) -> &'a mut Self {
            todo!()
        }

        pub fn compile(&self) -> Option<RedoxError> {

            let main_file_name: &str;
            match self.crate_type {

                CrateType::Lib => {
                    main_file_name = "lib.rs";
                },
                CrateType::Bin => {
                    main_file_name = "main.rs";
                },
                _ => {panic!("This Type isnt supported yet")}
            }
            let mut compile_command = Command::new("rustc");
            let _ = compile_command
                .args(&self.flags[..])
                .arg(self.root.clone() + PATH_SEPERATOR + &self.src_dir + PATH_SEPERATOR + main_file_name);

            let mut child = match compile_command.spawn() {
                Ok(value) => {value},
                Err(_) => {return Some(RedoxError::Error)}
            };

            match child.wait() {
                Ok(_) => {None},
                Err(_) => {Some(RedoxError::Error)},
            }
        }

        pub fn make_bin(&mut self) -> &mut Self {
            self.crate_type = match &self.crate_type {
                CrateType::Lib => {CrateType::Bin},
                CrateType::Bin => {CrateType::Bin},
                CrateType::CargoLib => {CrateType::CargoBin},
                CrateType::CargoBin => {CrateType::CargoBin},
                CrateType::RedoxManagedBin => {CrateType::RedoxManagedBin},
                CrateType::RedoxManagedLib => {CrateType::RedoxManagedBin},
                CrateType::Empty => {panic!("Cant change an empty crate to a binary! (fn make_bin)")}
            };
            self
        }

        pub fn make_lib(&mut self) -> &mut Self {
            self.crate_type = match &self.crate_type {
                CrateType::Lib => {CrateType::Lib},
                CrateType::Bin => {CrateType::Lib},
                CrateType::CargoLib => {CrateType::CargoLib},
                CrateType::CargoBin => {CrateType::CargoLib},
                CrateType::RedoxManagedBin => {CrateType::RedoxManagedLib},
                CrateType::RedoxManagedLib => {CrateType::RedoxManagedLib},
                CrateType::Empty => {panic!("Cant change an empty crate to a library! (fn make_bin)")}
            };
            self
        }
    }

    pub struct Redoxr {
        crates: Vec<RustCrate>,
        common_flags: Vec<String>,
        main: RustCrate,
        crate_type: CrateType,
        build_status: Option<RedoxError>,
        cli_args: EmptyField,
    }
    
    impl Redoxr {
        pub fn new() -> Self {
            let mut build_script = Self {
                crates: Vec::new(),
                common_flags: Vec::new(),
                main: RustCrate::empty(),
                crate_type: CrateType::Bin,
                build_status: None,
                cli_args: EmptyField,
            };
            if let Some(error) = Redoxr::setup_env() {error.panic()}
            if let Some(status) = build_script.self_compile() {build_script.build_status = Some(status);}
            build_script
        }

        ///Changes behaviour based on the command_given
        pub fn build(&mut self) -> Option<RedoxError> {
            self.compile_rest()
        }

        ///This Method will compile the rest of the files that havent been compiled yet
        ///and extern the into the main crate, which is compiled at the end
        pub fn compile_rest (&mut self) -> Option<RedoxError> {
            panic!("This is not fully implemented yet and shouldnt be used! (fn compile_rest)");

            //let mut compile_command = Command::new("rustc");
            //if let Some(error) = self.get_all_deps() {return Some(error);}
            //let main_crate = &self.main;
            //let _ = compile_command
            //    .arg(main_crate.root.clone() + "/" + &main_crate.src_dir + "/main.rs")
            //    .args(&["-O"]).spawn().unwrap().wait();

            //None
        }
        pub fn get () -> bool {
            true
        }
        fn get_all_deps (&self) -> Option<RedoxError> {
            None
        }

        fn setup_env() -> Option<RedoxError> {
            let mut command = Command::new("mkdir");
            let _ = command.args(&["-p", &("bin".to_owned() + PATH_SEPERATOR + "deps")]);
            let mut child = match command.spawn() {
                Ok(value) => {value},
                Err(_) => {return Some(RedoxError::Error)},
            };
            match child.wait() {
                Ok(_) => {None},
                Err(_) => {Some(RedoxError::Error)},
            }
        }

        fn self_compile(&self) -> Option<RedoxError> {
            let mut compile_command = Command::new("rustc");
            let _ = compile_command.arg("build.rs");
            let mut child = match compile_command.spawn() {
                Ok(value) =>  {
                    value
                },
                Err(_value) => {
                    return Some(RedoxError::Error);
                }
            };
            match child.wait() {
                Ok(_value ) => {
                    None
                },
                Err(_value ) => {
                    Some(RedoxError::Error)
                }
            }
        }

        fn set_main (&mut self, main: RustCrate) -> &mut RustCrate {
            self.main = main;
            &mut self.main
        }
        fn add_crate (&mut self, lib: RustCrate) -> &mut RustCrate {
            let index = self.crates.len();
            self.crates.push(lib);
            &mut self.crates[index]
        }
    }
}

pub mod oxygen_cli {

    #[cfg(target_os = "linux")]
    pub const PATH_SEPERATOR: &'static str = r"/";
    
    #[cfg(target_os = "windows")]
    pub const PATH_SEPERATOR: &'static str = r"\";

    #[derive(Clone)]
    enum OxygenCommandType {
        WithArg,
        NoArg,
        ParentCommand,
        HelpFlag,
    }

    #[derive(Clone)]
    enum OxygenIOArgType {
        LongFlag,
        ShortFlag,
        Command,
        UserInput,
        EndOfArgs,
    }

    #[derive(Clone)]
    struct OxygenIOArg (pub String, pub OxygenIOArgType);

    #[derive(Clone)]
    struct OxygenCommand<A>
        where A: FnMut() -> (){
            name: String,
            command_type: OxygenCommandType,
            description: String,
            action: A,
            children: Vec<OxygenCommand<A>>,
    }

    impl <A> OxygenCommand<A>
    where A: FnMut() -> () {
        pub fn new(name: &str, action: A, command_type: OxygenCommandType, description: String) -> Self {
            Self { name: name.to_string(), action, command_type, description, children: Vec::new() }

        }
    }

    #[derive(Clone)]
    struct OxygenFlag {
        pub used: bool,
        pub arg: String,
        description: String,
        long: String,
        short: String,
        flag_type: OxygenCommandType,
    }

    #[derive(Clone)]
    pub struct OxygenCLI<A>
        where A: FnMut() -> () {
            commands: Vec<OxygenCommand<A>>,
            help_flag: String,
            flags: Vec<OxygenFlag>,
        }

    impl<A> OxygenCLI<A>
    where A: FnMut() -> () {
        pub fn new() -> Self {
            let mut return_struct = Self{
                commands: Vec::new(),
                help_flag: "".to_owned(),
                flags: Vec::new(),
            };

            let _ = return_struct.flag();
            return_struct

        }
        pub fn arg(&mut self, name: &str, action: A) -> &mut Self {
            self.commands.push(
                OxygenCommand::new(name, action, OxygenCommandType::NoArg, "No description provided".to_string())
            );
            self
        }
        pub fn flag(&mut self) -> &mut Self {
            self
        }

        fn help(&mut self) -> () {
            println!("Commands:");
            for command in &self.commands {
                println!("\t{} - {}", command.name, command.description);
            }

            println!("\nFlags:");
            for flag in &self.flags {
                println!("\t{} {} - {}", flag.long, flag.short, flag.description);
            }
        }

        fn get_args () -> Vec<OxygenIOArg> {
            let raw_args = std::env::args().collect::<Vec<String>>();
            let mut output = Vec::new();
            for arg in raw_args {
                if arg.len() > 0 {
                    if arg == "--help" || arg == "-h" {

                        output.push(OxygenIOArg("help".to_string(), OxygenIOArgType::Command));

                    } else if arg.starts_with("--") {

                        output.push(OxygenIOArg(arg[2..].to_string(), OxygenIOArgType::LongFlag));

                    } else if arg.starts_with("-") {

                        output.push(OxygenIOArg(arg[1..].to_string(), OxygenIOArgType::ShortFlag));

                    } else {

                        output.push(OxygenIOArg(arg, OxygenIOArgType::Command));

                    }
                } else {
                    output.push(OxygenIOArg("".to_string(), OxygenIOArgType::EndOfArgs));
                }
            }
            output
        }

        pub fn run() -> bool {
            true
        }
    }

}

pub mod truck {
    #[cfg(target_os = "linux")]
    pub const PATH_SEPERATOR: &'static str = r"/";
    
    #[cfg(target_os = "windows")]
    pub const PATH_SEPERATOR: &'static str = r"\";

    use std:: {
        env,
        fs,
        path::Path,
        path::PathBuf,
        ffi::OsString
    };

    pub struct Truck {
        out_dir: OsString,
    }

    impl Truck {
        pub fn new () -> Self {
            Self {
                out_dir: env::var_os("OUT_DIR").unwrap(),
            }
        }
        pub fn rerun(self) -> Self {
            println!("cargo::rerun-if-changed=build.rs");
            self
        }
        pub fn get_out_dir(&self) -> &OsString {
            &self.out_dir
        }
        pub fn add_cargo_setting (self, setting: &str, value: &str) -> Self {
            let setting_to_print = "cargo::".to_owned() + setting + "=" + value;
            println!("{}", setting_to_print);
            self
        }
    }

    pub struct TruckFile(PathBuf, String);

    impl TruckFile {
        pub fn new(out_dir: &Truck, name: &str) -> Self  {
            Self (
                Path::new(out_dir.get_out_dir()).join(name),
                "".to_owned()
            )
        }
        pub fn write (mut self, value: &str) -> Self {
            self.1 = value.to_owned();
            fs::write(&self.0,value).unwrap();
            self
        }
    }

}
