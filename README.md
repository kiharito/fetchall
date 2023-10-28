# fetchall

This CLI app runs `git fetch` on all registered directories at once.

## Installation

You can install this app using cargo.
After [installing cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html),
```shell
cargo install --git https://github.com/kiharito/fetchall
```

You can also download the binary files directly.
[See Assets in the Release](https://github.com/kiharito/fetchall/releases/latest).

## Usage
### Add a directory to the targets
```shell
fetchall add ~/workspaces/rails_tutorial/
```
Relative path is also available.
```shell
fetchall add .
```

### Run `git fetch` in each of the target directories
```shell
fetchall exec
```
[Git fetch options](https://git-scm.com/docs/git-fetch#_options) may be specified.
```shell
fetchall exec -p
```

### Show the list of the target directories
```shell
fetchall ls
# 0: /Users/username/workspaces/rails_tutorial
# 1: /Users/username/workspaces/fetchall
```

### Remove the directory from the targets
Specify the index displayed by `fetchall ls`.
```shell
fetchall rm 0
```
