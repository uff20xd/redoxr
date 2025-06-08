use std::{
    process::{
        Command,
    },
};

struct OxUtils;

impl OxUtils {
}

pub struct RedOxR {
    file_name: String,
    dir: String,
    target: String,
    rustc_flags: Vec<String>,
    crate_type: String,
    output_file: String,
    is_main: bool,
    libraries: Vec<[String; 2]>
}

impl RedOxR {
    pub fn self_build(&mut self) -> &mut Self{
        self.add_library("redoxr", "libredoxr.rlib")
    }

    pub fn new (name: &str) -> Self {
        Self {
            file_name: name.to_owned() + ".rs",
            dir: ".".to_owned(),
            target: "x86_64-unknown-linux-gnu".to_owned(),
            rustc_flags: Vec::new(),
            crate_type: "bin".to_owned(),
            output_file: name.to_owned(),
            is_main: false,
            libraries: Vec::new(),
        }
    }

    pub fn compile (&mut self) -> &mut Self {
        let mut compiling_command = Command::new("rustc");
        let mut compiling_command = compiling_command.current_dir(&self.dir)
            .arg(&self.file_name)
            .arg("--crate-type=".to_owned() + &self.crate_type)
            .arg("--target=".to_owned() + &self.target);

        if self.is_main {
            let mut compiling_command = compiling_command.arg("--crate-name").arg(&self.output_file);
        }
        for crates in &self.libraries {
            let mut compiling_command = compiling_command.arg("--extern").arg(crates[0].clone() + "=" + &crates[1]);
        }
        for flag in &self.rustc_flags {
            let mut compiling_command = compiling_command.arg(flag);
        }
        let temp = compiling_command.spawn().unwrap().wait();
        self
    }

    pub fn add_library (&mut self, name: &str, path: &str) -> &mut Self {
        self.libraries.push([name.to_owned(), path.to_owned()]);
        self
    }

    pub fn set_crate_type (&mut self, crate_type: &str) -> &mut Self {
        self.crate_type = crate_type.to_owned();
        self
    }

    pub fn generate_crate (&mut self) -> &mut Self {
        self.is_main = true;
        self
    }

    pub fn set_system_target (&mut self, target: &str) -> &mut Self {
        self.target = target.to_owned();
        self
    }


    pub fn set_output (&mut self, file: &str) -> &mut Self {
        self.output_file = file.to_owned();
        self
    }

    pub fn add_flag(&mut self, flag: &str) -> &mut Self {
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
