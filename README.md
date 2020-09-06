# lsproj

A small no-dependancy single file pure python script to find project roots. Use with fzf for maximum effect.

# Usage

Make sure lsproj is somewhere in `PATH`, then use it to cd into project files quickly.


```
alias cdp = (lsproj ~/Documents ~/dev |column -t | fzf --ansi --preview='onefetch -d {4}' --preview-window=up:30 --border -n 3)
```
