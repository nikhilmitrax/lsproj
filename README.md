# lsproj - Project Finder and Selector

A Rust utility for finding and selecting projects on your computer, with the ability to quickly navigate to the selected project directory. `lsproj` is proudly built using the Rust programming language.

## Features

- Recursively searches specified directories for projects
- Identifies projects based on common marker files (package.json, Cargo.toml, etc.)
- Uses skim for fuzzy finding and project selection
- Shows a preview of the project directory contents
- **Automatically changes to the selected project directory** when used with the proper alias

## Installation

1. Build the executable:
   ```bash
   cargo build --release
   ```

2. Make sure the executable is in your PATH. You can either:
   - Copy the binary to a directory in your PATH: `cp target/release/lsproj /usr/local/bin/`
   - Or add the release directory to your PATH: `export PATH=$PATH:/path/to/lsproj/target/release`

## Shell Integration

For optimal usage, add this alias to your shell configuration:

```bash
# For Bash/Zsh (.bashrc or .zshrc)
# Important: Note that we don't need additional quotes around the command substitution
alias cdp='cd $(lsproj ~/dev ~/workspace)'

# For Fish shell (config.fish)
alias cdp='cd (lsproj ~/dev ~/workspace)'
```

Alternative approach using a shell function for better path handling:

```bash
# For Bash/Zsh (.bashrc or .zshrc)
function cdp() {
  local dir="$(lsproj ~/dev ~/workspace)"
  if [ -n "$dir" ]; then
    cd "$dir"
  fi
}

# For Fish shell (config.fish)
function cdp
  set dir (lsproj ~/dev ~/workspace)
  if test -n "$dir"
    cd "$dir"
  end
end
```

Customize the directories according to your project locations. This allows you to quickly navigate to any project by simply typing `cdp` and using fuzzy search.

## Usage

Once you've set up the alias, simply run:

```bash
cdp
```

This will:
1. Search through your configured directories (~/dev and ~/workspace in the example) for projects
2. Present a list of found projects using skim with fuzzy search
3. Allow you to select a project using arrow keys and Enter
4. Automatically change to the selected project directory

You can also run `lsproj` directly to just get the path of a selected project:

```bash
lsproj ~/dev ~/projects
```

## Project Detection

Projects are identified by common marker files with the following prefixes shown in the list:
- package.json (JavaScript/Node.js) - /
- Pipfile, pyproject.toml, requirements.txt (Python) - P
- Cargo.toml (Rust) - R
- meson.build, CMakeLists.txt (C++) - C
- .git repositories without other markers (Generic)

## License

MIT
