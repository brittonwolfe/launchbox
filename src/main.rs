use std::{
	fs::read_to_string,
	io,
	env::{current_dir, set_current_dir},
	path::Path,
	process::{Command, Stdio}
};

use termion::{
	clear,
	event::Key,
	input::TermRead,
	raw::IntoRawMode
};

use toml::Value;

use tui::{
	Terminal,
	backend::TermionBackend,
	layout::{Layout, Constraint, Direction},
	style::{Modifier, Style},
	widgets::{Block, Borders, List, ListItem, ListState, Paragraph}
};

fn main() -> Result<(), io::Error> {
	println!("starting launchbox");
	let stdout = io::stdout().into_raw_mode()?;
	let backend = TermionBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	let mut stdin = termion::async_stdin().keys();

	println!("loading configuration...");
	// propagate up until we find a .launchbox file
	let i_dir = current_dir().unwrap();
	let mut dir = i_dir.as_path();
	let mut i_path = dir.join(".launchbox");
	let mut path = i_path.as_path();
	let root = Path::new("/");
	while !path.exists() {
		if dir == root {
			// can't go past root. perish
			println!("no configuration found!");
			return Ok(());
		}
		dir = dir.parent().unwrap();
		i_path = dir.join(".launchbox");
		path = i_path.as_path();
	}
	println!("found configuration");
	set_current_dir(dir)?;
	// load our config file and work with the data
	let i_toml_str = read_to_string(path).unwrap();
	let toml_str = i_toml_str.as_str();
	let i_config: Value = toml::from_str(toml_str).unwrap();
	let config = i_config.as_table().unwrap();
	let i_sec_main = config.get("launchbox").unwrap();
	let sec_main = i_sec_main.as_table().unwrap();
	let i_i_category = sec_main.get("category").unwrap();
	let i_category: &Vec<Value> = i_i_category.as_array().unwrap();
	// i want this to be an array in the future for optimization
	let mut category = Vec::new();
	category.push("All".to_string());
	for entry in i_category {
		category.push(entry.as_str().unwrap().to_string());
	}

	// build our command list
	let mut exe = Vec::new();
	let mut all_exe = Vec::new();
	let mut list = Vec::new();
	let mut all_list = Vec::new();
	exe.push(Vec::new());
	list.push(Vec::new());
	for entry in &category[..] {
		let mut e_exe = Vec::new();
		let mut e_list = Vec::new();
		let i_sec = config.get(entry.as_str());
		if i_sec.is_none() {
			println!("{}", entry);
			continue;
		}
		let sec = i_sec.unwrap().as_table().unwrap();
		for key in sec.keys() {
			let i_val = sec.get(key.as_str()).unwrap();
			let val = i_val.as_str().unwrap();
			let tuple = (key, val);
			let item = ListItem::new(key.as_str());
			let a_item = ListItem::new(key.as_str());
			e_exe.push(tuple);
			e_list.push(item);
			all_exe.push(tuple);
			all_list.push(a_item);
		}
		exe.push(e_exe);
		list.push(e_list);
	}
	exe[0] = all_exe;
	list[0] = all_list;
	let cat_count = category.len();

	// init state
	let mut cat = 0;
	let mut sel = 0;
	//let mut pos = 0;

	println!("{}{}", clear::All, termion::cursor::Hide);

	'logic: loop {
		let exe_count = exe[cat].len();
		let content: &[ListItem] = &list[cat][..];
		let list = List::new(content)
			.block(Block::default().title(category[cat].as_str()).borders(Borders::ALL))
			.highlight_style(Style::default().add_modifier(Modifier::BOLD))
			.highlight_symbol("> ");
		let mut state = ListState::default();
		state.select(Some(sel));
		// Render content
		terminal.draw(|f| {
			let chunks = Layout::default().direction(Direction::Horizontal).margin(1).constraints(
				[
					Constraint::Percentage(40),
					Constraint::Percentage(60)
				].as_ref()
			).split(f.size());
			f.render_stateful_widget(list, chunks[0], &mut state);
			let block = Block::default().title("Info").borders(Borders::ALL);
			f.render_widget(block, chunks[1]);
		})?;

		// Handle input
		let input = stdin.next();
		if let Some(Ok(key)) = input {
			match key {
				Key::Char('Q') |
				Key::Ctrl('c')	=>	break 'logic,
				// Enter keypress
				Key::Char('\n')	|
				Key::Char(' ')	=>	{
					let command = exe[cat][sel].1;
					let mut split = command.split(" ");
					println!("{}", termion::cursor::Show);
					Command::new(split.next().unwrap())
					.args(split)
					.current_dir(dir)
					.stdout(Stdio::null())
					.spawn()?;
				}, //execute command at [cat:sel]
				Key::Up |
				Key::Char('w')	=>	sel -= if sel != 0 { 1 } else { 0 },
				Key::Down |
				Key::Char('s')	=>	sel += if sel != exe_count { 1 } else { 0 },
				Key::Left |
				Key::Char('a')	=>	{
					cat -= if cat != 0 { 1 } else { 0 };
					sel = 0;
				},
				Key::Right |
				Key::Char('d')	=>	{
					cat += if cat != (cat_count - 1) { 1 } else { 0 };
					sel = 0;
				},
				_				=>	(),
			}
		}
	}
	println!("{}", termion::cursor::Show);
	Ok(())
}
