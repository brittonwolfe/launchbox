use std::{
	fs::read_to_string,
	io,
	env::{current_dir, set_current_dir},
	path::Path
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
	style::{Color, Modifier, Style},
	text::{Span, Spans},
	widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap}
};

mod child;
use child::Subprocess;

fn main() -> Result<(), io::Error> {
	println!("starting launchbox");
	let stdout = io::stdout().into_raw_mode()?;
	let backend = TermionBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;
	let mut stdin = termion::async_stdin().keys();

	println!("loading configuration...");
	// propagate up the fs until we find a .launchbox file
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
	let i_sec_info = sec_main.get("info").unwrap();
	let sec_info = i_sec_info.as_table().unwrap();

	// build our command list
	let mut exe = vec![Vec::new()];
	let mut all_exe = Vec::new();
	let mut list = vec![Vec::new()];
	let mut all_list = Vec::new();
	for entry in &category[..] {
		let mut e_exe = Vec::new();
		let mut e_list = Vec::new();
		let i_sec = config.get(entry.as_str());
		if i_sec.is_none() {
			continue;
		}
		let sec = i_sec.unwrap().as_table().unwrap();
		for key in sec.keys() {
			let i_val = sec.get(key.as_str()).unwrap();
			let val = i_val.as_str().unwrap();
			let tuple = (key, val);
			let item = ListItem::new(key.as_str());
			e_exe.push(tuple);
			e_list.push(item);
			all_exe.push(tuple);
		}
		exe.push(e_exe);
		list.push(e_list);
	}
	all_exe.sort();
	for item in &all_exe {
		let item = ListItem::new(item.0.as_str());
		all_list.push(item);
	}
	exe[0] = all_exe;
	list[0] = all_list;
	let cat_count = category.len();

	// init state
	let mut cat = 0;
	let mut sel = 0;
	// I want to control the offset myself, but ListState.offset
	// isn't pub so I'm putting it off until the crate updates,
	// or until I implement some logic to determine if the selected
	// option is out of range and move it myself.
	//let mut pos = 0;

	println!("{}{}", clear::All, termion::cursor::Hide);

	let mut subs: Vec<Subprocess> = Vec::new();
	'logic: loop {
		// kill all dead or zombie processes
		// and remove them from our subs Vec
		for n in 0..subs.len() {
			if !subs[n].alive() {
				subs.remove(n).kill();
			}
		}
		//	Waiting to fix process tracking: currently, some
		//	programs redirect to other processes or create
		//	their OWN children, which means we currently don't
		//	directly track their state. Not super necessary,
		//	but would be nice to have.
		let mut b_content = list[cat].clone();
		for n in 0..list[cat].len() {
			for proc in &subs {
				if proc.title.as_str() == exe[cat][n].0.as_str() {
					let tmp = b_content.remove(n).style(Style::default().fg(Color::Green));
					b_content.insert(n, tmp);
				}
			}
		}
		// Get our state and widgets ready
		let exe_count = exe[cat].len();
		let content: &[ListItem] = &b_content[..]; // switch this to b_content for process tracking
		let list = List::new(content)
			.block(Block::default().title(category[cat].as_str()).borders(Borders::ALL))
			.highlight_style(Style::default().add_modifier(Modifier::BOLD))
			.highlight_symbol("> ");
		let mut state = ListState::default();
		state.select(Some(sel));
		let selection = exe[cat][sel];
		// build info text
		let mut text = vec![
			Spans::from(Span::raw(format!("$ {}", selection.1).to_string()))
		];
		if sec_info.contains_key(selection.0) {
			let sel_info = sec_info[selection.0].as_array();
			for i_line in sel_info.unwrap() {
				let line = i_line.as_str().unwrap();
				let span = Spans::from(Span::raw(line));
				text.push(span);
			}
		}

		// Render content
		terminal.draw(|f| {
			let chunks = Layout::default().direction(Direction::Horizontal).margin(1).constraints(
				[
					Constraint::Percentage(40),
					Constraint::Percentage(60)
				].as_ref()
			).split(f.size());
			f.render_stateful_widget(list, chunks[0], &mut state);
			let info = Paragraph::new(text)
				.block(Block::default().title("Info").borders(Borders::ALL))
				.wrap(Wrap { trim: true });
			f.render_widget(info, chunks[1]);
		})?;

		// Handle input
		let input = stdin.next();
		if let Some(Ok(key)) = input {
			match key {
				Key::Char('Q') |
				Key::Ctrl('c')	=>	break 'logic,
				Key::Char('q') => {
					let target = selection.0;
					for n in 0..subs.len() {
						if subs[n].title.as_str() == target.as_str() {
							subs.remove(n).kill();
						}
					}
				},
				Key::Char('\n')	|
				Key::Char(' ')	=>	{
					let mut process = Subprocess::new(selection, dir.to_str().unwrap().to_string());
					process.start();
					subs.push(process);
				},
				Key::Up |
				Key::Char('w')	=>	sel -= if sel != 0 { 1 } else { 0 },
				Key::Down |
				Key::Char('s')	=>	sel += if sel != (exe_count - 1) { 1 } else { 0 },
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
