use std::{
	env::{current_dir, var},
	io::{BufReader, BufWriter, Error, Read, Result, Write},
	process::{Child, ChildStdin, ChildStdout, Command, Stdio}
};

use crate::child::Subprocess;

struct ShellInstance {
	inner: Subprocess
}

impl ShellInstance {
	fn new() -> ShellInstance {
		let home = var("HOME").unwrap();
		let shell = var("SHELL").unwrap();
		let sub_data = (&"".to_string(), shell.as_str());
		let inner = Subprocess::new(sub_data, home);
		let output = ShellInstance {
			inner
		};
		return output;
	}
	fn start(mut self) -> () {
		self.inner.start();
	}
	pub fn alive(self) -> bool {
		return self.inner.alive();
	}
	pub fn get_pipes(self) -> (BufWriter<ChildStdin>, BufReader<ChildStdout>) {
		let streams = self.inner.piped();
		return (BufWriter::new(streams.0), BufReader::new(streams.1));
	}
}
