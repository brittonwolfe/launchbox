use std::{
	path::Path,
	process::{Command, Child, Stdio}
};

pub struct Subprocess {
	title: String,
	inner: Command,
	source: String,
	child: Option<Child>,
	pid: Option<u32>
}

impl Subprocess {
	pub fn new(src: (&String, &str), cwd: String) -> Subprocess {
		let title = src.0.clone();
		let source = src.1.to_string();
		let mut split = source.split(" ");
		let dir = Path::new(cwd.as_str());
		// show cursor if applicable? needs more config
		//println!("{}", termion::cursor::Show);
		let mut command = Command::new(split.next().unwrap());
		command.args(split)
		.current_dir(dir)
		.stdout(Stdio::null())
		.stderr(Stdio::null());
		let output = Subprocess {
			title,
			inner: command,
			source,
			child: None,
			pid: None
		};
		return output;
	}
	pub fn start(&mut self) -> () {
		let child = self.inner.spawn().unwrap();
		self.pid = Some(child.id());
		self.child = Some(child);
	}
	pub fn alive(&mut self) -> bool {
		return false;
	}
	pub fn kill(self) -> () {
		// Yeah, we're just going to ignore the result lmao
		self.child.unwrap().kill().unwrap();
	}
}
