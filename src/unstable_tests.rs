    pub struct OxygenChild(Child);

    pub struct Oxygen {
        cmd: Vec<String>,
        dir: String,
    }

    impl Oxygen {
        pub fn command(command: &str) -> Self  {
            Self {
                cmd: vec![command.to_owned()],
                dir: ".".to_owned(),
            }
        }
        pub fn add_arg(&mut self, arg: &str) -> &mut Self {
            self.cmd.push(arg.to_owned());
            self
        }
        pub fn add_args(&mut self, args: &[&str]) -> &mut Self {
            for arg in args {
                self.cmd.push(arg.to_string());
            }
            self
        }
        pub fn reset_args(&mut self) -> &mut Self {
            let temp = self.cmd[0].clone();
            self.cmd = vec![temp];
            self
        }
        pub fn reset (&mut self) -> &mut Self {
            self.cmd = vec![];
            self
        }
        pub fn run (&self) -> bool {
            let mut command = Command::new(&self.cmd[0]);
            command.args(&self.cmd[1..]);
            let _child = command.spawn();
                //.unwrap();
            //OxygenChild(child)
            true
        }
    }

