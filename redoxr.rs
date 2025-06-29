//====================================================================
//Code that handles it. Like slightly easier to use than the library version.
//====================================================================

#![allow(dead_code)]
pub mod redoxr {
    use std::{
        env, fs, path::Path, process::{
            exit, Command, //Child,
        }, str::{self}, sync::Arc
    };
 
    #[derive(Debug, Clone)]
    enum CrateBuilder {
        SingleFile,
        Cargo,
        RedOxR,
        PreBuilt,
        None,
    }

    enum CrateType {
        Lib,
        Bin,
    }

    // Implement Concept for better builds
    
    pub struct RedoxCrate {
        name: String,
        path: String,
        external: Option<String>,
        src_dir: String,
        refrence_counter: u64,
    }

    /// Struct that defines a crate for as the main file or a dependency
    impl RedoxCrate {
        pub fn empty() -> Self {
            Self {
                name: "".to_owned(),
                path: "".to_owned(),
                external: None,
                src_dir: "".to_owned(),
                refrence_counter: 0
            }
        }

        pub fn main<'a>(script: &'a mut Redoxr, src_dir: &str) -> &'a mut Self {
            let mut call = Self {
                name: "main".to_owned(),
                path: ".".to_owned(),
                external: None,
                src_dir: src_dir.to_owned(),
                refrence_counter: 0
            };
            script.add_main(call)
        }

        pub fn compile(&self) {

        }
    }

    pub enum RedoxArgs {
        Build,
        Run,
        Get,
        Custom(Option<String>),
    }

    pub struct Redoxr {
        name: String,
        dependencies: Vec<RedoxCrate>,
        rustc_flags: Vec<String>,
        main: RedoxCrate,
        crate_type: CrateType,
        build_status: bool,
        cli_args: RedoxArgs,
    }
    
    impl Redoxr {
        pub fn new(name: &str) -> Self {

            let args = env::args().collect::<Vec<String>>();
            let command = match args[0].to_lowercase().as_str() {
                "build" => {
                    if args.len() < 2 {
                        RedoxArgs::Build
                    }
                    else {
                        RedoxArgs::Build
                    }
                },
                "run" => {
                    if args.len() < 2 {
                        RedoxArgs::Run
                    }
                    else {
                        RedoxArgs::Run
                    }
                },
                "get" => {RedoxArgs::Get},
                _ => {
                    if args.len() < 2 {
                        RedoxArgs::Custom(None)
                    }
                    else {
                        RedoxArgs::Custom(Some(args[1].clone()))
                    }
                }
            };
            let mut build_script = Self {
                name: name.to_owned(),
                dependencies: Vec::new(),
                rustc_flags: Vec::new(),
                main: RedoxCrate::empty(),
                crate_type: CrateType::Bin,
                build_status: true,
                cli_args: command,
            };
            build_script.build_status = build_script.compile_build_script();
            build_script
        }
        pub fn build (&mut self) -> bool {
            match &self.cli_args {
                RedoxArgs::Build => {
                    self.compile()
                },
                RedoxArgs::Run => {
                    true
                },
                RedoxArgs::Get => {
                    true
                },
                RedoxArgs::Custom(_value) => {
                    panic!("Option is not known!");
                }
            }
        }
        pub fn compile (&mut self) -> bool {
            let mut compile_command = Command::new("rustc");
            if !self.get_all_deps() {return false;}
            let main_crate = &self.main;
            let _ = compile_command.arg(main_crate.path.clone() + "/" + &main_crate.src_dir + "/main.rs").args(&["-O"]).spawn().unwrap().wait();
            true
        }
        pub fn get () -> bool {
            true
        }
        fn get_all_deps (&self) -> bool {
            true
        }
        fn compile_build_script(&self) -> bool {
            let mut command = Command::new("rustc");
            let mut child = command.arg("build.rs").spawn().unwrap();
            match child.wait() {
                Ok(_value ) => {
                    true
                },
                Err(_value ) => {
                    false
                }
            }
        }
        fn add_main(&mut self, main: RedoxCrate) -> &mut RedoxCrate {
            self.main = main;
            &mut self.main
        }
        fn add_lib (&mut self, lib: RedoxCrate) -> &mut RedoxCrate {
            let index = self.dependencies.len();
            self.dependencies.push(lib);
            &mut self.dependencies[index]
        }
    }
}

pub mod oxygen_cli {

    enum OxygenCommandType {
        WithArg,
        NoArg,
        ParentCommand,
        HelpFlag,
    }

    enum OxygenIOArgType {
        LongFlag,
        ShortFlag,
        Command,
        EndOfArgs,
    }

    struct OxygenIOArg (pub String, pub OxygenIOArgType);

    #[derive(Clone)]
    struct OxygenCommand<A>
        where A: FnMut() -> (){
            name: String,
            command_type: OxygenCommandType,
            description: String,
            action: T,
            children: Vec<OxygenCommand>,
    }

    impl <A> OxygenCommand<A>
    where A: FnMut() -> () {
        pub fn new(name: &str, action: A) -> Self {
            Self { name: name.to_string(), action}

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
            help_flag
            flags: Vec<OxygenFlag>,
        }

    impl<A> OxygenCLI<A>
    where T: FnMut() -> () {
        pub fn new() -> Self {
            let mut return_struct = Self(Vec::new());
            return_struct.flag()

        }
        pub fn arg(&mut self, name: &str, action: A) -> &mut Self {
            self.commands.push(OxygenCommand::new(name, action));
            self
        }
        pub fn flag(&mut self) -> &mut Self {
            self
        }

        fn help(&mut self) -> &mut Self {
            println!("Commands:");
            for command in self.commands {
                println!("\t{command.name} - {command.description}");
            }


            println!("\nFlags:");
            for flag in self.flags {
                println!("\t{flag.long} {flag.short} - {flag.description}");
            }
        }

        fn get_args () -> Vec<OxygenIOArg> {

        }

        pub fn run() -> bool {
            true
        }
    }

}

pub mod truck {
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
