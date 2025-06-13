//====================================================================
//Code that handles it. Like slightly easier to use than the library version.
//====================================================================

#![allow(dead_code)]
pub mod redoxr {
    use std::{
        fs,
        path::Path,
        process::{
            exit, Command, //Child,
        }, 
        str::{self},
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
    }

    impl RedoxCrate {
        pub fn empty() -> Self {
            Self {
                name: "".to_owned(),
                path: "".to_owned(),
                external: None,
                src_dir: "".to_owned()
            }
        }

        pub fn main<'a>(script: &'a mut Redoxr, src_dir: &str) -> &'a mut Self {
            let mut call = Self {
                name: "main".to_owned(),
                path: ".".to_owned(),
                external: None,
                src_dir: src_dir.to_owned()
            };
            script.add_lib(call)
        }
    }

    pub struct Redoxr {
        name: String,
        dependencies: Vec<RedoxCrate>,
        rustc_flags: Vec<String>,
        main: RedoxCrate,
        crate_type: CrateType,
        build_status: bool,
    }
    
    impl Redoxr {
        pub fn new(name: &str) -> Self {
            let mut build_script = Self {
                name: name.to_owned(),
                dependencies: Vec::new(),
                rustc_flags: Vec::new(),
                main: RedoxCrate::empty(),
                crate_type: CrateType::Bin,
                build_status: true
            };
            build_script.build_status = build_script.compile_build_script();
            build_script
        }
        fn compile_build_script(&self) -> bool {
            let mut command = Command::new("rustc");
            let mut child = command.arg("build.rs").spawn().unwrap();
            match child.wait() {
                Ok(mut value ) => {
                    true
                },
                Err(value ) => {
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
