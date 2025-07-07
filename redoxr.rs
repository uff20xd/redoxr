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
//
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
//You can add dependencies just by using .dependency(dependency).
//(For this you currently need to compile the dependency first):
//
//####build.rs####
//mod redoxr;
//use redoxr::redoxr::*;
//
//fn main() -> () {
//    let mut redoxr = Redoxr::new();
//    let mut dependency = RustCrate::from_cargo(&mut redoxr, "clap");
//
//    if let Some(error) = dependency.compile() {error.panic()}
//
//    let mut main_crate = RustCrate::main(&mut redoxr, "some_crate").
//        .dependency(dependecy.clone());
//
//    //There is also this macro that compiles dependencies.
//    compile!(main_crate);
//}
//################
//
//The compile! macro just expands to the whole if-let-then-panic-statement.
//
//Redoxr-buildscripts usually automatically rebuild themselves, when the
//Redoxr-struct is constructed. This can be disabled by passing the --cfg no_rebuild flag.
//====================================================================

#![allow(dead_code)]

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
 
    #[derive(Clone, Debug)]
    pub struct Mirror<T> (*mut T);
    
    impl<T> Mirror<T> {
        pub fn new(pointer: *mut T) -> Self {
            Self(pointer)
        }
        pub fn borrow(&self) -> &T {
            unsafe {
                &(*(self.0))
            }
        }
        pub fn borrow_mut(&mut self) -> &mut T {
            unsafe {
                &mut (*(self.0))
            }
        }
        pub fn defer(self) {
            let _ = self;
        }
    }

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
        Error,
        WrongCrateType,
        NotExecutable,
        NotCompiled,
        AlreadyCompiled(String),
    }
    impl RedoxError {
        pub fn panic (&self) -> () {
            dbg!(self);
            panic!("");
        }
    }

    #[derive(Clone, Debug)]
    enum CrateType {
        Lib,
        Bin,
        Empty,
    }

    #[derive(Clone, Debug)]
    enum CrateManager {
        Redoxr,
        ExternalRedoxr,
        Cargo,
    }

    pub trait RedoxrCompatible {
        fn compile(&self) -> Option<RedoxError>;
        fn dependency<T>(&mut self, dep: T) -> &mut Self
            where T: RedoxrCompatible;
        //fn get_outputs(&self) -> String;
        fn is_output_file(&self) -> bool;
        fn is_compiled(&self) -> bool;
        fn is_bin(&self) -> bool;
        fn is_lib(&self) -> bool;
        fn get_outpath(&self) -> String;
        fn get_name(&self) -> String;
    }
    // Implement Concept for better builds
    

    ///A Struct that defines a Rust Crate managed by any build system
    #[derive(Clone, Debug)]
    pub struct RustCrate<'a> {
        name: String,
        root: String,
        src_dir: String,
        main_file: String,
        output_file: String,
        is_output_crate: bool,

        deps: Vec<Mirror<RustCrate<'a>>>,
        crate_type: CrateType,
        crate_manager: CrateManager,

        flags: Vec<String>,
        compiled: bool,

        refrence_counter: u64,


        //currently unused
        id: u64,
        external: Option<String>,
    }

    ///A macro so you don't have to type out the entire if-let-statement.
    ///Takes the crate to compile as input.
    #[macro_export]
    macro_rules! compile {
        ($comp_file:ident) => {
            if let Some(error) = ($comp_file).compile() {error.panic()}
        }
    }

    ///Basically the same as the compile! macro
    #[macro_export]
    macro_rules! run {
        ($comp_file:ident) => {
            if let Some(error) = ($comp_file).run() {error.panic()}
        }
    }

    /// Struct that defines a crate for as the main file or a dependency
    impl<'a> RustCrate<'a> {
        pub fn empty() -> Self {
            let call = Self {
                name: "".to_owned(),
                root: "".to_owned(),
                src_dir: "".to_owned(),
                main_file: "".to_owned(),
                output_file: "".to_owned(),
                is_output_crate: false,

                deps: Vec::new(),
                crate_type: CrateType::Empty,
                crate_manager: CrateManager::Redoxr,
                compiled: false,

                flags: Vec::new(),

                id: 0,
                refrence_counter: 0,

                external: None,
            };
            call
        }

        pub fn new(name: &str, root: &str) -> Self {
            let call = Self {
                name: name.to_owned(),
                root: root.to_owned(),
                src_dir: "src".to_owned(),
                main_file: "main.rs".to_owned(),
                output_file: name.to_owned(),
                is_output_crate: false,

                deps: Vec::new(),
                crate_type: CrateType::Lib,
                crate_manager: CrateManager::Redoxr,
                compiled: false,

                flags: Vec::new(),

                id: 0,
                refrence_counter: 0,

                external: None,
            };
            call
        }

        pub fn stay(&mut self) -> Self {
            self.to_owned()
        }

        pub fn from_cargo(_name: &str) -> Self {
            todo!()
        }

        pub fn compile(&mut self) -> Option<RedoxError> {
            if self.is_compiled() {return Some(RedoxError::AlreadyCompiled(self.name.clone()))}

            let output_path: String;
            if self.is_output_crate {
                output_path = "bin".to_owned() + PATH_SEPERATOR + &self.output_file;
            } else {
                output_path = "bin".to_owned() + PATH_SEPERATOR + "deps" + PATH_SEPERATOR + &self.output_file;
            }
            
            let crate_type;
            if self.is_bin() {
                crate_type = "bin".to_owned();
            } else {
                crate_type = "lib".to_owned();
            }

            let mut dependency_flags: Vec<(String, String)> = Vec::new();
            for dependency in &self.deps {
                if !dependency.borrow().is_compiled() {return Some(RedoxError::Error)}
                let dep = dependency.borrow();
                dependency_flags.push(( dep.name.clone(), dep.get_outpath()));

                #[cfg(debug)]
                dbg!(&dependency);
            }

            let mut compile_command = Command::new("rustc");
            let _ = compile_command
                .args(&self.flags[..])
                .arg(self.root.clone() + PATH_SEPERATOR + &self.src_dir + PATH_SEPERATOR + &self.main_file)
                .args(&["-o", &output_path])
                .args(&["-L", "bin/deps", "-L", "bin/"])
                .args(&["--crate-type", &crate_type]);

            for dependency in dependency_flags {
                let _ = compile_command
                    .args(&["--extern", &(dependency.0.clone() + "=" + &dependency.1)]);

                #[cfg(debug)]
                dbg!(&dependency);
            }

            #[cfg(debug)]
            dbg!(&compile_command);

            let mut child = match compile_command.spawn() {
                Ok(value) => {value},
                Err(_) => {return Some(RedoxError::Error)}
            };

            match child.wait() {
                Ok(_) => {
                    self.compiled = true;
                    None
                },
                Err(_) => {Some(RedoxError::Error)},
            }
        }

        pub fn is_compiled(&self) -> bool {
            self.compiled
        }

        pub fn is_bin(&self) -> bool {
            match self.crate_type {
                CrateType::Bin => {true},
                _ => {false}
            }
        }

        pub fn is_lib(&self) -> bool {
            match self.crate_type {
                CrateType::Lib => {true},
                _ => {false}
            }
        }

        pub fn make_output(&mut self) -> &mut Self {
            self.is_output_crate = true;
            self
        }

        pub fn make_bin(&mut self) -> &mut Self {
            self.crate_type = match &self.crate_type {
                CrateType::Lib => {CrateType::Bin},
                CrateType::Bin => {CrateType::Bin},
                CrateType::Empty => {panic!("Cant change an empty crate to a binary! (fn make_bin)")}
            };
            self.output_file = self.name.clone();
            self
        }

        ///This function is not meant to be used as RustCrates start as a lib
        pub fn make_lib(&mut self) -> &mut Self {
            self.crate_type = match &self.crate_type {
                CrateType::Lib => {CrateType::Lib},
                CrateType::Bin => {CrateType::Lib},
                CrateType::Empty => {panic!("Cant change an empty crate to a library! (fn make_bin)")}
            };
            self
        }

        pub fn depend_on(&mut self, dep: &'a mut RustCrate<'a>) -> &mut Self {
            self.deps.push(Mirror(dep));
            self
        }

        pub fn set_root(&mut self, new_root: &str) -> &mut Self {
            self.root = new_root.to_owned();
            self
        }

        pub fn set_src(&mut self, new_src: &str) -> &mut Self {
            self.src_dir = new_src.to_owned();
            self
        }

        pub fn set_main(&mut self, new_main: &str) -> &mut Self {
            self.main_file = new_main.to_owned();
            self
        }
        
        pub fn set_output_file(&mut self, new_output: &str) -> &mut Self {
            self.output_file = new_output.to_owned();
            self
        }

        pub fn is_output_file(&self) -> bool {
            self.is_output_crate
        }

        pub fn get_outpath (&self) -> String {
            let output_path: String;
            if self.is_output_file() {
                output_path = "bin".to_owned() + PATH_SEPERATOR + &self.output_file;
            } else {
                output_path = "bin".to_owned() + PATH_SEPERATOR + "deps" + PATH_SEPERATOR + &self.output_file;
            }
            output_path
        }

        pub fn get_name (&self) -> String {
            self.name.clone()
        }

        pub fn run(&self) -> Option<RedoxError> {
            if !self.is_compiled() {return Some(RedoxError::NotCompiled)}
            if !self.is_bin() {return Some(RedoxError::NotExecutable)}

            let command_name = ".".to_owned() + PATH_SEPERATOR + &self.get_outpath();
            let mut run_command = Command::new(command_name);
            let mut child = match run_command.spawn() {
                Ok(value) => {value},
                Err(_) => {return Some(RedoxError::Error)}
            };

            match child.wait() {
                Ok(_) => {None},
                Err(_) => {Some(RedoxError::Error)}
            }
        }
    }

    ///Basically the same as the 
    #[macro_export]
    macro_rules! handle {
        ($comp_file:ident, $method:ident) => {
            if let Some(error) = ($comp_file).$method() {error.panic()}
        }
    }

    pub struct Redoxr<'a> {
        flags: Vec<&'a str>,
        cli_args: EmptyField,
    }
    
    impl Redoxr<'_> {
        pub fn new() -> Self {
            #[allow(unused_mut)]
            let mut build_script = Self {
                flags: Vec::new(),
                cli_args: EmptyField,
            };
            build_script
        }

        pub fn setup_env(&self) -> Option<RedoxError> {
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

        pub fn debug(mut self) -> Self {
            self.flags.push("--cfg");
            self.flags.push("debug");
            self
        }

        pub fn self_compile(&self) -> Option<RedoxError> {
            let mut compile_command = Command::new("rustc");
            let _ = compile_command.arg("build.rs")
                .args(&self.flags[..]);

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
