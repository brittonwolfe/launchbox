#  Launchbox

Launchbox is my first Rust application. I prefer to work from the terminal, so I started launchbox to launch applications from a TUI. It's configurable, and one configuration can serve any subdirectories.

## What Launchbox Does

Launchbox is a tool launcher that runs in the command line. I previously used something like `run` from my [shell scripts](https://github.com/brittonwolfe/shell-scripts) to put together easy scripts for things like `run build`, `run last`, etc., but wanted to make a TUI application because I think they're cool.

## Packing your Launchbox

```toml
[launchbox]
category = [] # your categories go here
[Category] # the same as in the launchbox.category field
name = "some command" #name is what you want to see in your menu, "some command" is what you want it to execute!
[launchbox.info]
name = [
	"some super informative information",
	"that describes what this thing does"
]
# info can be as many lines as you want!
```



## Roadmap

Launchbox is still in its early stages, but its "core" functionality is implemented. Here's my roadmap for working on it in the future:

- [x] Loads a `toml` formatted configuration file
- [x] Displays items in a list and launches them
- [x] Sorts items in the "all" view
- [x] Shows basic information in the info pane (name and command)
- [x] Shows user-defined data in the info pane (install location, description, etc.)
- [ ] Scrollable info pane
- [ ] Allows the user switch the Info pane to an output tab that shows stdout and stderr from spawned processes (stdin too?)
- [ ] a global config file to load defaults from
- [ ] Shell pane/integration so I don't have to back out of launchbox?