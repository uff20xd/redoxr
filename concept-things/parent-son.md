use Redoxr

fn main() -> () {
	let registstry = Registry::new("https://github.com");

	let project1 = RedOxR::build("Project_name")
		.add_flag(&["--target", "x86-64"]);

	let crate1 = RedoxCrate::main(&mut project1, ".", "src");
	let crate2 = RedoxCrate::extern(&mut project, &mut registry, "uff20xd/redox");
	
	let action = match project1.get_cli {
		RedoxCli::Build => |x| { project1.compile }
	}
	if !action { project1.error() }
}
