//====================================================================
//Code that handles it. Like slightly easier to use than the library version.
//====================================================================
#![allow(dead_code)]
pub mod redoxr {
    use std::{
        process::{
            Command,
        },
        path::Path,
        fs
    };
    struct Library(pub CrateBuilder, pub String, pub String);

    enum CrateBuilder {
        SingleFile,
        Cargo,
        RedOxR,
        PreBuilt,
        None,
    }

    pub struct RedOxR {
        file_name: String,
        dir: String,

        target: String,
        rustc_flags: Vec<String>,
        crate_type: String,
        output_file: String,
        libraries: Vec<Library>,

        external_address: String,
        crate_builder: CrateBuilder,
    }
    
    impl RedOxR {
        pub fn self_build(self) -> Self{
            self.add_rlib("redoxr", "libredoxr.rlib").set_dir(".")
        }

        pub fn new (name: &str) -> Self {
            Self {
                file_name: name.to_owned() + ".rs",
                dir: "src".to_owned(),

                target: "x86_64-unknown-linux-gnu".to_owned(),
                rustc_flags: Vec::new(),
                crate_type: "bin".to_owned(),
                output_file: name.to_owned(),

                libraries: Vec::new(),

                external_address: "".to_owned(),
                crate_builder: CrateBuilder::None,
            }
        }

        pub fn external (name: &str, address: &str) -> Self {
            Self {
                file_name: name.to_owned() + ".rs",
                dir: name.to_owned(),

                target: "x86_64-unknown-linux-gnu".to_owned(),
                rustc_flags: Vec::new(),
                crate_type: "bin".to_owned(),
                output_file: name.to_owned(),

                libraries: Vec::new(),

                external_address: address.to_owned(),
                crate_builder: CrateBuilder::None,
            }
        }

        pub fn compile (self) -> Self {
            let mut compiling_command = Command::new("rustc");
            println!("{} {}", &self.dir, &self.file_name);
            compiling_command.current_dir(&self.dir)
                .arg(&self.file_name)
                .arg("--crate-type=".to_owned() + &self.crate_type)
                .arg("--target=".to_owned() + &self.target)
                .arg("-o".to_owned()).arg(&self.output_file);

            for crates in &self.libraries {
                compiling_command.arg("--extern").arg(crates.1.clone() + "=" + &crates.2);
            }

            for flag in &self.rustc_flags {
                compiling_command.arg(flag);
            }

            let _temp = compiling_command.spawn().unwrap().wait();
            self
        }

        pub fn run(self, args: &str) -> Self {
            let mut command = Command::new("./".to_owned() + &self.output_file);
            let args_new = args.split_whitespace();
            let _child = command
                .current_dir(&self.dir)
                .args(args_new)
                .spawn().unwrap();
            self
        }

        pub fn set_dir(mut self, dir: &str) -> Self {
            self.dir = dir.to_owned();
            self
        }

        pub fn add_rlib(mut self, name: &str, path: &str) -> Self {
            self.libraries.push(Library(CrateBuilder::PreBuilt, name.to_owned(), path.to_owned()));
            self
        }

        fn get_git(&mut self) -> Library {
            let mut git_command = Command::new("git");
            git_command.arg("clone");
            let mut child = git_command.arg(&self.external_address).arg(&self.output_file).spawn().unwrap();
            let _ = child.wait();
            let builder = {

            };
            Library(CrateBuilder::None,self.output_file.clone() ,self.dir.clone())
        }

        pub fn add_lib(mut self, mut lib: Self) -> Self {
            if !Path::new("git_reps").exists() {
                let _ = fs::create_dir("git_reps");
            }
            &
            self.libraries.push(lib.get_git());
            self
        }

        pub fn set_crate_type (mut self, crate_type: &str) -> Self {
            self.crate_type = crate_type.to_owned();
            self
        }

        pub fn set_system_target (mut self, target: &str) -> Self {
            self.target = target.to_owned();
            self
        }

        pub fn set_output (mut self, file: &str) -> Self {
            self.output_file = file.to_owned();
            self
        }

        ///This is a Debug Function used for selfbuilding
        pub fn copy_raw(self, path: &str) -> Self {
            let mut command = Command::new("cp");
            let _child = command
                .arg("-u")
                .arg("-p")
                .arg(&self.file_name)
                .arg(path.to_owned() + "/" + &self.file_name)
                .spawn()
                .unwrap();

            self
        }

        ///Implement later
        pub fn compile_c (self) -> Self {
            todo!();
        }

        pub fn reset_flags (mut self) -> Self {
            self.rustc_flags = Vec::new();
            self
        }

        pub fn add_flag(mut self, flag: &str) -> Self {
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

pub mod redoxr_cargo {
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
