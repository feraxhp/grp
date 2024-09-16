# grp (git repository)

girep is a command line tool that helps you manage your git repositories in the cloud, for difrent platforms. 
It is written in Rust, and it is inspired by [gcli](https://github.com/herrhotzenplotz/gcli).

## Features
- [x] List repositories
- [x] Create repositories
- [x] Delete repositories

## Supported Platforms

- [x] Github
- [x] Gitea
- [ ] Gitlab
- [ ] Jetbrains Space
- 

## Installation

- Windows: Download the latest .exe from the releases page. and add it to your PATH.
- Ubuntu and fedora: See realeases page for the latest deb and rpm packages.
- Arch Linux: See build instructions below.
- Other Linux distributions: See build instructions below.
- MacOS: See build instructions below.


## Build and Run

### Dependencies

Make sure you have the following dependencies installed on your system:
![dependencies](dependencies.md)

- Cargo (Rust)

### Build
```bash

# clone the repository
git clone https://github.com/feraxhp/cim.git
cd cim

# if you want to install it on your system
cargo install --path .
```

## Need more Functionality?

If you need more functionality, feel free to open an issue or a pull request.
remember to follow the [contribution guidelines](CONTRIBUTING.md)