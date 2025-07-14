#![allow(dead_code)]
///Redoxr is a open-source build_scripter inspired by [nob.h](https://github.com/tsoding/nob.h) and
///[Cargo](https://github.com/rust-lang/cargo).
///
///Its not made to be used in actual projects and does not fit many use cases.
///On the contrary it does help with actually thinking about what you are building and what libs you
///really need.
///There is also (planned) cross-compatitibility with Cargo both ways. 
///Cargo to Redox will be added first and Redox to Cargo will be added using Truck (see at the
///bottom).
///All outputs will be put in the bin/ (and bin/deps/) directory, which sanitizes the
///environment and adds reproducability.
///
///This is just the start and in the end I want to be able to compile anything using Redoxr and also
///be able to easily extend it to any other language using "intelligent trait usage".
///
///Basic Usage:
///
///```Rust
///mod redoxr;
///use redoxr::redoxr::*;
///
///fn main() -> () {
///    let mut redoxr = Redoxr::new();
///
///    let mut main_crate = RustCrate::main( "some_crate");
///    if let Some(error) = main_crate.compile() {panic!("{}",error)};
///}
///```
///
///You can add dependencies just by using .depend_on(dependency).
///(For this you currently need to compile the dependency first):
///
///```Rust
///mod redoxr;
///use redoxr::redoxr::*;
///
///fn main() -> () {
///    let mut redoxr = Redoxr::new();
///    let mut dependency = RustCrate::from_cargo("clap", "clap");
///
///    if let Some(error) = dependency.compile() {panic!("{}",error)}
///
///    let mut main_crate = RustCrate::main(&mut redoxr, "some_crate").
///        .depend_on(dependecy.clone());
///
///    //There is also this macro that compiles dependencies.
///    compile!(main_crate);
///}
///```
///
///The compile! macro just expands to the whole if-let-then-panic-statement.
///
///The dependency list uses the incredible Mirror technology (just a semi-safe wrapper around raw
///pointer), which allows for dependencies to be built after being passed as such while still being
///able to handle errors.
///Currently that is most of the magic, but soon I will implement the version that uses traits,
///making it expandable to other languages easily as long as I write it correctly.

pub mod redoxr {

    #[cfg(not(comp_version_2024))]
    #[cfg(not(comp_version_2024))]
    #[cfg(not(comp_version_2024))]
    #[cfg(not(comp_version_2024))]
    pub const COMP_VERSION: &[&str] = &[];

    #[cfg(comp_version_2024)]
    pub const COMP_VERSION: &[&str] = &["--edition", "2024"];

    #[cfg(comp_version_2021)]
    pub const COMP_VERSION: &[&str] = &["--edition", "2021"];

    #[cfg(comp_version_2018)]
    pub const COMP_VERSION: &[&str] = &["--edition", "2018"];

    #[cfg(comp_version_2015)]
    pub const COMP_VERSION: &[&str] = &["--edition", "2015"];

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
        fs::self,
        error::Error,
        fmt::Display,
        fmt::Formatter,
        path::Path,
    };

    type IOError = std::io::Error;
    pub type Cmd = Command;
    pub type AnyError = Box<dyn std::error::Error>;
    pub type MainResult = Result<(), AnyError>;
 
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

    #[derive(Debug)]
    pub enum RedoxError {
        Error(u32),
        WrongCrateType(String, String),
        NotExecutable,
        NotCompiled,
        AlreadyCompiled(String),
        IOProcessFailed(IOError),
    }

    impl Display for RedoxError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            let output = match self {
                RedoxError::Error(line) => {format!("Currently Undefined Error! line: {}:{}", file!(), line)},
                RedoxError::WrongCrateType(error, error2) => {format!("The Crate is of the wrong type: {}! Should be {}", error, error2)},
                RedoxError::NotExecutable => {format!("The File is not executable!")},
                RedoxError::NotCompiled => {format!("The File is not Compiled!")},
                RedoxError::AlreadyCompiled(file) => {format!("File {} is already compiled!", file)},
                RedoxError::IOProcessFailed(error) => {format!("{}", error)},
            };
            write!(f, "{}", output)
        }
    }
    impl Error for RedoxError {}

    impl From<IOError> for RedoxError {
        fn from(io_error: std::io::Error) -> Self {
            RedoxError::IOProcessFailed(io_error)
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
        Prebuilt,
    }

    pub trait RedoxrCompatible {
        fn compile(&self) -> Result<(),RedoxError>;
        fn depend_on(&mut self, dep: *mut Box<dyn RedoxrCompatible>) -> &mut Box<dyn RedoxrCompatible>;
        //fn get_outputs(&self) -> String;
        fn flags(&mut self) -> &mut Box<dyn RedoxrCompatible>;
        fn is_output_file_name(&self) -> bool;
        fn is_compiled(&self) -> bool;
        fn is_bin(&self) -> bool;
        fn is_lib(&self) -> bool;
        fn get_outpath(&self) -> String;
        fn get_name(&self) -> String;
        fn get_root(&self) -> String;
        fn stay(&mut self) -> Box<dyn RedoxrCompatible>;
    }
    // Implement Concept for better builds
    
    ///A macro so you don't have to type out the entire if-let-statement.
    ///Takes the crate to compile as input.
    #[macro_export]
    macro_rules! compile {
        ($comp_file:ident) => {
            _ = ($comp_file).compile()?;
        }
    }

    ///Basically the same as the compile! macro
    #[macro_export]
    macro_rules! run {
        ($comp_file:ident) => {
            _ = ($comp_file).run()?;
        }
    }

    ///A Struct that defines a Rust Crate managed by any build system
    #[derive(Clone, Debug)]
    pub struct RustCrate<'a> {
        name: String,
        root: String,
        src_dir: String,
        main_file: String,
        output_file: String,
        is_output_crate: bool,
        show_output: bool,

        pass_on_args: Vec<String>,
        deps: Vec<Mirror<RustCrate<'a>>>,
        crate_type: CrateType,
        crate_manager: CrateManager,

        flags: Vec<&'a str>,
        compiled: bool,

        //refrence_counter: u64,


        //currently unused
        //id: u64,
        //external: Option<String>,
    }

    ///Represents a Rust Crate with refrences to all dependencies.
    impl<'a> RustCrate<'a> {
        pub fn empty() -> Self {
            let call = Self {
                name: "".to_owned(),
                root: "".to_owned(),
                src_dir: "".to_owned(),
                main_file: "".to_owned(),
                output_file: "".to_owned(),
                is_output_crate: false,
                show_output: false,

                pass_on_args: Vec::new(),

                deps: Vec::new(),
                crate_type: CrateType::Empty,
                crate_manager: CrateManager::Redoxr,
                compiled: false,

                flags: Vec::new(),
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
                show_output: false,

                pass_on_args: Vec::new(),

                deps: Vec::new(),
                crate_type: CrateType::Lib,
                crate_manager: CrateManager::Redoxr,
                compiled: false,

                flags: Vec::new(),

                //id: 0,
                //refrence_counter: 0,

                //external: None,
            };
            call
        }

        pub fn stay(&mut self) -> Self {
            self.to_owned()
        }

        pub fn from_cargo(name: &str, root: &str) -> Self {
            let call = Self {
                name: name.to_owned(),
                root: root.to_owned(),
                src_dir: "src".to_owned(),
                main_file: "main.rs".to_owned(),
                output_file: name.to_owned(),
                is_output_crate: false,
                show_output: false,

                pass_on_args: Vec::new(),

                deps: Vec::new(),
                crate_type: CrateType::Lib,
                crate_manager: CrateManager::Cargo,
                compiled: false,

                flags: Vec::new(),
            };
            call
        }

        ///Compiles Cargo Crates and moves their output into out/deps/
        pub fn compile_cargo(&mut self) -> Result<(), RedoxError> {
            let crate_type;
            if self.is_bin() {
                crate_type = "bin".to_owned();
            } else {
                crate_type = "lib".to_owned();
            }

            let mut compile_command = Command::new("cargo");
            let _ = compile_command
                .current_dir(&self.root)
                .arg("build")
                .arg("--release")
                .args(&self.flags[..]);
                //.args(&["--crate-type", &crate_type]);

            #[cfg(debug)]
            dbg!(&compile_command);

            let mut child = compile_command.spawn()?;
            let _ = child.wait()?;

            let mut _needed_files: Vec<String> = Vec::new();

            let release_path = self.root.clone() + PATH_SEPERATOR + "target" + PATH_SEPERATOR + "release";
            let name = "lib".to_owned() + &(self.name.clone()) + ".rlib";

            let mut copy_command = Command::new("cp");

            let _ = copy_command
                //.arg()
                .arg(release_path.clone() + &name)
                .arg("out/deps");
            _ = self.set_output_file(&name);

            Ok(())
        }

        pub fn compile(&mut self) -> Result<(), RedoxError> {

            let input_path = self.root.clone() + PATH_SEPERATOR + &self.src_dir + PATH_SEPERATOR + &self.main_file;
            let output_path: String;
            if self.is_output_crate {
                output_path = "out".to_owned() + PATH_SEPERATOR + &self.output_file;
            } else {
                output_path = "out".to_owned() + PATH_SEPERATOR + "deps" + PATH_SEPERATOR + &self.output_file;
            }

            println!("\nCompiling {}: {} -> {}", self.name.clone(), self.root.clone(), &output_path);
            if self.is_compiled() {return Err(RedoxError::AlreadyCompiled(self.name.clone()))}
            if self.is_cargo() {return self.compile_cargo()}
            
            let crate_type;
            if self.is_bin() {
                crate_type = "bin".to_owned();
            } else {
                crate_type = "lib".to_owned();
            }

            let mut dependency_flags: Vec<(String, String, String)> = Vec::new();
            for dependency in &self.deps {
                if !dependency.borrow().is_compiled() {return Err(RedoxError::Error(line!()))}
                let dep = dependency.borrow();
                dependency_flags.push(( dep.get_name(), dep.get_outpath(), dep.get_root()));

                #[cfg(debug)]
                dbg!(&dependency);
            }

            let mut compile_command = Command::new("rustc");
            _ = compile_command
                .args(COMP_VERSION)
                .args(&self.flags[..])
                .arg(&input_path)
                .args(&["-o", &output_path])
                .args(&["-L", "bin/deps", "-L", "bin/"])
                .args(&["--crate-type", &crate_type]);

            for dependency in dependency_flags {
                let _ = compile_command
                    .args(&["--extern", &(dependency.0.clone() + "=" + &dependency.1)]);
                self.add_perma_args(&[
                    "-L", &(dependency.2.clone() + PATH_SEPERATOR + "target" + PATH_SEPERATOR + "release"),
                    "-L", &(dependency.2.clone() + PATH_SEPERATOR + "target" + PATH_SEPERATOR + "release" + PATH_SEPERATOR + "deps")
                ]);

                #[cfg(debug)]
                dbg!(&dependency);
            }

            _ = compile_command
                .args(&self.pass_on_args[..]);

            #[cfg(debug)]
            dbg!(&compile_command);

            if self.is_show_output() {
                let mut child = compile_command.spawn()?;

                match child.wait() {
                    Ok(_) => {
                        self.compiled = true;
                        return Ok(());
                    },
                    Err(_) => {return Err(RedoxError::Error(line!()))},
                }
            } else {
                _ = compile_command.output()?;
                return Ok(());
            }
        }

        pub fn is_compiled(&self) -> bool {
            self.compiled
        }

        pub fn is_cargo(&self) -> bool {
            match self.crate_manager {
                CrateManager::Cargo => {true},
                _ => {false}
            }
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
            self.show_output = true;
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

        pub fn depend_on(&mut self, dep: *mut RustCrate<'a>) -> &mut Self {
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

        pub fn set_show_output(&mut self, new_state: bool) -> &mut Self {
            self.show_output = new_state;
            self
        }

        pub fn add_perma_args(&mut self, args: &[&str]) -> &mut Self {
            for arg in args {
                self.pass_on_args.push(arg.to_string());
            }
            self
        }

        pub fn is_output_file(&self) -> bool {
            self.is_output_crate
        }

        pub fn is_show_output (&self) -> bool {
            self.show_output
        }

        pub fn get_outpath (&self) -> String {
            let output_path: String;
            if self.is_output_file() {
                output_path = "out".to_owned() + PATH_SEPERATOR + &self.output_file;
            } else {
                output_path = "out".to_owned() + PATH_SEPERATOR + "deps" + PATH_SEPERATOR + &self.output_file;
            }
            output_path
        }

        pub fn get_name (&self) -> String {
            self.name.clone()
        }

        pub fn get_root (&self) -> String {
            self.root.clone()
        }

        pub fn flags (&mut self, flags: &[&'a str]) -> &mut Self {
            for flag in flags {
                self.flags.push(flag);
            }
            self
        }

        ///Only used for the purpose of distributing my one wonder and debugging.
        ///You probably shouldnt use this.
        pub fn copy_raw(&self, path: &str) -> Result<(),RedoxError> {
            //#[cfg(target_os = "windows")]
            //let mut copy_command = Command::new("copy");

            #[cfg(target_os = "linux")]
            let mut copy_command = Command::new("cp");

            let _ = copy_command
                //.arg()
                .arg(self.root.clone() + PATH_SEPERATOR + &self.src_dir + PATH_SEPERATOR + &self.main_file)
                .arg(path);

            _ = copy_command.status()?;
            Ok(())
        }

        ///runs the compiled crate as long as the --cgf run option is enabled
        pub fn run(&self) -> Result<(), RedoxError> {
            #[cfg(not(run))]
            const RUN: bool = false;

            #[cfg(run)]
            const RUN: bool = true;

            if !RUN { return Ok(()) }
            if !self.is_compiled() {return Err(RedoxError::NotCompiled)}
            if !self.is_bin() {return Err(RedoxError::NotExecutable)}

            let command_name = ".".to_owned() + PATH_SEPERATOR + &self.get_outpath();
            let mut run_command = Command::new(command_name);
            _ = run_command.status()?;
            Ok(())
        }
    }

    pub struct Redoxr<'a> {
        flags: Vec<&'a str>,
        crates: Vec<Mirror<RustCrate<'a>>>,
        cli_args: EmptyField,
        compiled: bool,
    }
    
    impl<'a> Redoxr<'a> {
        pub fn new(flags: &[&'a str]) -> Self {
            #[allow(unused_mut)]
            let mut build_script = Self {
                flags: Vec::new(),
                cli_args: EmptyField,
                crates: Vec::new(),
                compiled: false,
            };

            for flag in flags {
                build_script.flags.push(flag);
            }

            #[cfg(not(manual))]
            #[cfg(boot_strap)]
            {
                let _ = build_script.flags.push("--cfg");
                let _ = build_script.flags.push("boot_strap");
                _ = match build_script.self_compile() {
                    Ok(_) => {},
                    Err(err) => {panic!("{}", err)},
                };
            }

            #[cfg(not(manual))]
            if let Some(_) = build_script.setup_env() {}

            build_script
        }

        pub fn flags(mut self, flags: &[&'a str]) -> Self {
            for flag in flags {
                self.flags.push(flag);
            }
            self
        }

        pub fn cfg(mut self, flag: &'a str) -> Self {
            self.flags.push("--cfg");
            self.flags.push(flag);
            self
        }

        pub fn setup_env(&self) -> Option<RedoxError> {
            let mut command = Command::new("mkdir");
            let _ = command.args(&["-p", &("out".to_owned() + PATH_SEPERATOR + "deps")]);
            let mut child = match command.spawn() {
                Ok(value) => {value},
                Err(_) => {return Some(RedoxError::Error(line!()))},
            };
            match child.wait() {
                Ok(_) => {None},
                Err(_) => {Some(RedoxError::Error(line!()))},
            }
        }

        pub fn add_crates(&mut self, crates: &[*mut RustCrate<'a>]) -> &mut Self {
            for rcrate in crates {
                self.crates.push(Mirror::new(rcrate.clone()));
            }
            self
        }

        pub fn auto_compile(&mut self)  -> &mut Self {
            todo!()
        }

        pub fn generate_json(&self) -> Option<RedoxError> {
            todo!();
        }

        pub fn debug(mut self) -> Self {
            self.flags.push("--cfg");
            self.flags.push("debug");
            self
        }

        pub fn self_compile(&mut self) -> Result<(), RedoxError> {

            #[cfg(boot_strap)]
            #[cfg(manual)]
            const BOOT_STRAP: bool = true;

            #[cfg(not(manual))]
            #[cfg(boot_strap)]
            const BOOT_STRAP: bool = true;

            #[cfg(manual)]
            #[cfg(not(boot_strap))]
            const BOOT_STRAP: bool = true;

            #[cfg(not(manual))]
            #[cfg(not(boot_strap))]
            const BOOT_STRAP: bool = false;

            if self.compiled || !BOOT_STRAP { return Ok(()) }
            else { self.compiled = true; }

            let mut compile_command = Command::new("rustc");
            let _ = compile_command.arg("build.rs")
                .args(COMP_VERSION)
                .args(&self.flags[..]);

            #[cfg(not(quiet))]
            let _ = compile_command.status()?;

            #[cfg(quiet)]
            let _ = compile_command.output()?;

            Ok(())
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
