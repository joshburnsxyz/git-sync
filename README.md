# git-sync
Shortcut to pull updates for a repo, and then push local changes, tags, etc.

## Usage

Simply run `git sync` inside of your repo to run the script.

```
$ cd my-repo
$ git sync
```

## What it does?

The script will first perform a `git pull`, followed by `git push`. It will then run `git push --tags`
to ensure any new tags are pushed upstream aswell. 

## Installation

### From Source

1. Clone repo - `git clone https://github.com/joshburnsxyz/git-sync`
2. Change directory into repo - `cd git-sync`
3. Run installer - `sudo make install`
