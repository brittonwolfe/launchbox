#  Launchbox

Launchbox is my first Rust application. I prefer to work from the terminal, so I started launchbox to launch applications from a TUI. It's configurable, and one configuration can serve any subdirectories.

## What Launchbox Does

Launchbox is a tool launcher that runs in the command line. I previously used something like `run` from my [shell scripts](https://github.com/brittonwolfe/shell-scripts) to put together easy scripts for things like `run build`, `run last`, etc., but wanted to make a TUI application because I think they're cool.

## Roadmap

Launchbox is still in its early stages, but its "core" functionality is implemented. Here's my roadmap for working on it in the future:

- [x] Loads a `toml` formatted configuration file
- [x] Displays items in a list and launches them
- [x] Sorts items in the "all" view
- [ ] Shows basic information in the info pane (name and command)
- [ ] Shows user-defined data in the info pane (install location, description, etc.)
- [ ] Allows the user switch the Info pane to an output tab that shows stdout and stderr from spawned processes (stdin too?)
- [ ] a global config file to load defaults from
- [ ] Shell pane/integration so I don't have to back out of launchbox?

### Smaller Features

- [ ] resource checking in the info tab using `sysinfo::Process`