use std::{
	path::Path,
	process::{Command, Child, ChildStdin, ChildStdout, Stdio}
};

use procinfo::pid::{stat, State};

pub struct Subprocess {
	pub title: String,
	inner: Command,
	//source: String,
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
			//source,
			child: None,
			pid: None
		};
		return output;
	}
	pub fn start(&mut self) -> () {
		let child = self.inner.spawn().unwrap();
		self.pid = Some(child.id());
		self.child = Some(child);
		while !self.alive() {}
	}
	pub fn alive(&self) -> bool {
		if self.pid.is_none() {
			return false;
		}
		let pid: i32 = self.pid.unwrap() as i32;
		let info = stat(pid).unwrap();
		match info.state {
			State::Dead |
			State::Stopped |
			State::Zombie	=>	return false,
			_				=>	return true
		}
	}
	pub fn kill(self) -> () {
		// We're just going to ignore the child process's dying screams
		self.child.unwrap().kill().ok();
	}
	pub fn piped(mut self) -> (ChildStdin, ChildStdout) {
		self.inner.stdin(Stdio::piped());
		self.inner.stdout(Stdio::piped());
		let child = self.child.unwrap();
		let i = child.stdin;
		let o = child.stdout;
		return (i.unwrap(), o.unwrap());
	}
}
