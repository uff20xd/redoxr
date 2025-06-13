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
    struct Library{
        pub crate_builder: CrateBuilder,
        pub build_flags: Vec<String>,
        pub name: String,
        pub path: String,
        pub output_file: String,
    }
    impl Library {
        fn new (name: &str, path: &str, output_file: &str, crate_builder: CrateBuilder, build_flags: &[&str]) -> Self {
            let mut flags = Vec::new();
            for flag in build_flags {
                flags.push(flag.to_string());
            }
            let return_self = Self { 
                crate_builder: crate_builder,
                build_flags: flags,
                name: name.to_owned(),
                path: path.to_owned(),
                output_file: output_file.to_owned()
            };
            return_self
        }
    }

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

    #[derive(Debug)]
    pub struct RedOxR {
        name: String,
        main_file: String,
        out_name: String,

        root_dir: String,
        src_dir: String,
        out_dir: String,

        rustc_flags: Vec<String>,

        crate_type: String,
        libraries: Vec<Library>,

        external_address: String,
        crate_builder: CrateBuilder,
        is_external: bool,
        get_from_output: Vec<String>,

        //processes: Vec<Child>,
    }
    
    impl RedOxR {
        pub fn build_script() -> Self {
            let mut build_script = Self {
                name: "build".to_owned(),
                main_file: "build.rs".to_owned(),
                out_name: "build".to_owned(),

                rustc_flags: Vec::new(),

                root_dir: ".".to_owned(),
                src_dir: ".".to_owned(),

                crate_type: "bin".to_owned(),
                out_dir: ".".to_owned(),

                libraries: Vec::new(),

                external_address: "".to_owned(),
                crate_builder: CrateBuilder::SingleFile,
                is_external: false,

                get_from_output: Vec::new(),
                //processes: Vec::new(),
            };
            build_script.libraries.push(Library::new("redoxr", ".", "", CrateBuilder::PreBuilt, &[]));
            build_script
        }

        pub fn root (name: &str, root: &str) -> Self {
            if name.contains("-") || root.contains("-") {
                println!("RedOxR-Error: Neither crate path nor crate name may contain a \"-\"!");
            }
            Self {
                name: name.to_owned(),
                main_file: "main.rs".to_owned(),
                out_name: name.to_owned(),

                rustc_flags: Vec::new(),

                root_dir: root.to_owned(),
                src_dir: "src".to_owned(),
                out_dir: "bin".to_owned(),

                crate_type: "bin".to_owned(),
                libraries: Vec::new(),

                external_address: "".to_owned(),
                crate_builder: CrateBuilder::RedOxR,
                is_external: false,
                
                get_from_output: vec![name.to_owned()],
            }
        }

        pub fn external (name: &str, address: &str) -> Self {
            let mut return_self = Self {
                name: name.to_owned(),
                main_file: "lib.rs".to_owned(),
                out_name: "".to_owned(),

                rustc_flags: Vec::new(),

                root_dir: ".".to_owned(),
                src_dir: "src".to_owned(),
                out_dir: "bin".to_owned(),

                crate_type: "dylib".to_owned(),

                libraries: Vec::new(),

                external_address: address.to_owned(),
                crate_builder: CrateBuilder::Cargo,
                is_external: true,

                get_from_output: vec![name.to_owned()],
            };
            let _ = return_self.set_crate_type("staticlib");
            return_self
        }

        pub fn error(&self) -> () {
            println!("This is an error: {:?}", &self);
            exit(1)
        }

        pub fn compile (&self) -> bool {
            let _ = fs::create_dir(self.root_dir.clone() + "/bin");
            let out_file = self.out_dir.clone() + "/" + &self.out_name;

            let mut compiling_command = Command::new("rustc");
            println!("main_file: {}/{}/{}",&self.root_dir, &self.src_dir, &self.main_file);
            //println!("libraries: {:?}", &self.libraries);

            compiling_command.current_dir(&self.root_dir)
                .arg(self.src_dir.clone() + "/" + &self.main_file)
                .arg("--crate-type=".to_owned() + &self.crate_type)
                .arg("-o".to_owned()).arg(out_file)
                .arg("-O");


            let mut mod_file = "".to_owned();
            for crates in &self.libraries {
                match crates.crate_builder {
                    CrateBuilder::PreBuilt => {
                        //let _test = crates.1.clone() + "=" + &crates.2 +"/lib" + &crates.1 +".rlib";
                        //println!("{}",test);
                        compiling_command.arg("--extern").arg(crates.name.clone() + "=" + &crates.path + "/" + &crates.output_file);
                    },
                    CrateBuilder::Cargo => {
                        let _ = fs::create_dir(&self.src_dir);
                        let _ = fs::create_dir(self.src_dir.clone() + "/libs");
                        let mut cargo_command = Command::new("cargo");
                        let mut cargo_child = cargo_command.current_dir(&crates.path).args(&["build", "--workspace", "--release" ]);
                        dbg!(&cargo_child);
                        let _ = cargo_child.spawn().unwrap().wait();


                        //replace with fs::copy

                        let mut copy = fs::copy(
                           self.root_dir.clone() + "/" + &crates.path + "/target/release/" + &crates.output_file,
                           self.root_dir.clone() + "/" + &self.src_dir + "/libs/" + &crates.output_file
                        );
                        dbg!(&copy);
                        let _ = copy.unwrap();

                        compiling_command.arg("--extern").arg(crates.name.clone() + "=" + &self.src_dir + "/libs/" + &crates.output_file);
                        dbg!(&compiling_command);
                    },
                    _ => todo!()
                }
                mod_file = mod_file + "pub extern crate " + &crates.name + ";\n";
            }

            if self.libraries.len() > 0 as usize {
                let _ = match self.crate_builder {
                    CrateBuilder::SingleFile => (),
                    _ => {
                        let temp = self.src_dir.clone() + "/libs/mod.rs";
                        let path = Path::new(&temp);
                        let _ = fs::write(&path, mod_file);
                    }
                };
            }

            for flag in &self.rustc_flags {
                compiling_command.arg(flag);
            }

            let temp = compiling_command.spawn();
                //.unwrap().wait();
            match temp {
                Ok(mut a) => {
                    let _ = a.wait();
                    return true;
                },
                Err(_) => false
            };
            true
        }

        pub fn run(&self, args: &[&str]) -> bool {
            let heh = self.root_dir.clone() + "/" + &self.out_dir + "/" + &self.out_name;
            //println!("{}", &heh);
            let mut command = Command::new(&heh);
            let child = command
                .current_dir(&self.root_dir)
                .args(args)
                .spawn();
            match child {
                Ok(mut a) => {
                    let _ = a.wait();
                    true
                },
                Err(_) => false,
            }
        }

        pub fn set_root_dir(&mut self, dir: &str) -> &mut Self {
            self.root_dir = dir.to_owned();
            self
        }

        pub fn set_src_dir(&mut self, dir: &str) -> &mut Self {
            self.src_dir = dir.to_owned();
            self
        }

        pub fn add_rlib(&mut self, name: &str) -> &mut Self {
            let library = Library::new(
                name, &(self.src_dir.to_owned() + "/libs"),
                &("lib".to_owned() + name + ".rlib"),
                CrateBuilder::PreBuilt, &[]
            );
            let _ = self.libraries.push(library);
            self
        }

        fn get_git(&mut self) -> Library {
            let mut git_command = Command::new("git");
            git_command.arg("clone");
            let child = git_command.current_dir("git_reps").arg(&self.external_address).arg(&self.name);
            dbg!(&child);
            let _ = child.spawn().unwrap().wait();

            let mut ls_command = Command::new("ls");
            let raw_output = ls_command.current_dir("git_reps/".to_owned() + &self.name).output().unwrap().stdout;
            let output = str::from_utf8(&raw_output).unwrap().to_owned();
            dbg!(&output);

            let builder = {
                if output.contains("Cargo.toml"){
                    self.set_crate_type("lib");
                    println!("lol 1");
                    CrateBuilder::Cargo
                }
                else if output.contains("libredoxr.rlib") && output.contains("redoxr.rs") {
                    self.set_crate_type("lib");
                    println!("lol 2");
                    CrateBuilder::RedOxR
                }
                else {
                    println!("lol 3");
                    exit(99);
                }
            };
            Library::new(
                &self.name,
                &("git_reps/".to_owned() + &self.name),
                &self.out_name,
                builder,
                &[]
            )
                //self.rustc_flags[..]
        }

        pub fn add_lib(&mut self,lib: &mut Self) -> &mut Self {
            //println!("test1: {:?}", &lib);
            if lib.is_external {
                //println!("test2: {:?}", &lib);
                let path = self.root_dir.clone() + "git_reps";
                //let _ = &mut lib.add_flag();
                if !Path::new(&path).exists() {
                    let _ = fs::create_dir("git_reps");
                }
                let _ = self.libraries.push(lib.get_git().clone());
                //println!("test3: {:?}", &lib);
            } else {
                //println!("test4: {:?}", &lib);
                let mut ls_command = Command::new("ls");
                let raw_output = ls_command.arg(&lib.root_dir).output().unwrap().stdout;
                let output = str::from_utf8(&raw_output).unwrap().to_owned();

                //println!("output: {:?}, {:?}", &output, &raw_output);
                let builder = {
                    if output.contains("Cargo.toml"){
                        lib.set_crate_type("lib");
                        CrateBuilder::Cargo
                    }
                    else if output.contains("libredoxr.rlib") || output.contains("redoxr.rs") {
                        CrateBuilder::RedOxR
                    }
                    else {
                        println!("Couldnt get the Crate Builder used for: {:?}", &lib);
                        exit(99)
                    }
                };
                let library = Library::new(&lib.name, &lib.root_dir, &lib.out_name, builder, &[]);
                let _ = self.libraries.push(library);
            }
            self
        }

        pub fn set_crate_type (&mut self, crate_type: &str) -> &mut Self {
            self.out_name = match crate_type {
                "lib" => "lib".to_owned() + &self.name + ".rlib",
                "bin" => self.name.clone(),
                "dylib" => "lib".to_owned() + &self.name + ".so",
                "staticlib" => "lib".to_owned() + &self.name + ".a",
                _ => todo!(),
            };
            self.crate_type = crate_type.to_owned();
            self
        }

        pub fn set_crate_builder (&mut self, builder: &str) -> &mut Self {
            self.crate_builder = match builder {
                "none" | "None" => CrateBuilder::None,
                "cargo" | "Cargo" => CrateBuilder::Cargo,
                "redoxr" | "RedOxR" | "Redoxr" => CrateBuilder::RedOxR,
                "single_file" | "single file" | "SingleFile" | "single-file" => CrateBuilder::SingleFile,
                _ => exit(100)
            };
            self
        }

        pub fn set_output (&mut self, file: &str) -> &mut Self {
            self.out_dir = file.to_owned();
            self
        }

        pub fn set_main_file (&mut self, name: &str) -> &mut Self {
            self.main_file = name.to_owned();
            self
        }

        ///This is a Debug Function used for selfbuilding
        pub fn copy_raw(&mut self, path: &str) -> &mut Self {
            let mut command = Command::new("cp");
            let _child = command
                .arg("-u")
                .arg("-p")
                .arg(self.out_dir.to_owned() + "/" + &self.out_name)
                .arg(path.to_owned() + "/" + &self.main_file)
                .spawn()
                .unwrap();

            self
        }

        ///Implement later
        pub fn compile_c (&mut self) -> &mut Self {
            todo!();
        }

        pub fn reset_flags (&mut self) -> &mut Self {
            self.rustc_flags = Vec::new();
            self
        }

        pub fn add_flag(&mut self, flag: &[&str]) -> &mut Self {
            for i in flag {
                self.rustc_flags.push(i.to_string());
            }
            self
        }

        pub fn add_flag_str(&mut self, flag: &str) -> &mut Self {
            let raw_flags: String = flag.to_owned();
            let mut new_flags: Vec<String> = Vec::new();
            let mut start_slice: usize = 0;
            let mut end_slice: usize = 0;
            let temp = raw_flags.chars().collect::<Vec<char>>();
            let _ = match temp[end_slice] {
                ' ' => {
                    loop {
                        match temp[end_slice] {
                            ' ' => {end_slice += 1;},
                            _ => {
                                start_slice = end_slice;
                                break;
                            }
                        }
                    }
                },
                _ => ()
            };

            while end_slice < temp.len() - 1 {
                end_slice += 1;
                let _ = match temp[end_slice] {
                    ' ' => {
                        let one_flag = (&raw_flags[start_slice..end_slice]).to_owned();
                        new_flags.push(one_flag);
                        start_slice = end_slice + 1;
                    },
                    '\'' => {
                        loop {
                            end_slice += 1;
                            match temp[end_slice] {
                                '\'' => {
                                    break;
                                },
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                };
            }
            end_slice += 1;
            let one_flag = (&raw_flags[start_slice..end_slice]).to_owned();
            new_flags.push(one_flag);
            let mut new_flags = new_flags.into_iter().filter(|x| { x != ""}).collect::<Vec<String>>();
            self.rustc_flags.append(&mut new_flags);
            self
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
