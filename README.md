# grp (git repository)

[![.github/workflows/Releases.yml](https://github.com/feraxhp/grp/actions/workflows/Releases.yml/badge.svg)](https://github.com/feraxhp/grp/actions/workflows/Releases.yml)
![WinGet Package Version](https://img.shields.io/winget/v/feraxhp.grp?style=flat&color=green)

girep is a command line tool that helps you manage your git repositories in the cloud, for difrent platforms. 
It is written in Rust, and it is inspired by [gcli](https://github.com/herrhotzenplotz/gcli).

## Features
- New: 🎉
- Supported: ✅
- Planed support: 🟥
- On development: 🔶
- Not available on the platform: ⭕

### Repositories

| Feature | GitHub | Gitea | GitLab | Azure DevOps |
|:-------:|:------:|:-----:|:------:|:-----------:|
| List    |   ✅   |   ✅   |   ✅   | 🟥            |       
| Create  |   ✅   |   ✅   |   ✅   | 🟥            |
| Delete  |   ✅   |   ✅   |   ✅   | 🟥            |

### Organizations
| Feature | GitHub | Gitea | GitLab | Azure DevOps |
|:-------:|:------:|:-----:|:------:|:-----------:|
| List    |   ✅   |   ✅  |   ✅   | ⭕            |
| Create  |   ✅`1`|   ✅  |   ✅`2`| ⭕            |
| Delete  |   ✅   |   ✅  |   ✅`3`| ⭕            |

### git integrations

> [!TIP]
> The git integrations supports __any platform__ 
> that allows you to send oAuth credentials

- ✅ Clone
    - [x] bare
    - [x] branch
    - [x] __url clone__ This is how to clone __any platform__
    - [x] **repo_structure** base clone example `gh:feraxhp/grp` 
- ✅ Push
    - [x] tags
    - [x] branches
    - [x] force
    - [x] all
    - [x] dry-run
    - [x] set-upstream 
- ✅ Pull
    - [x] force: **Overrides any local code**
    - [x] rebase: Do a `git pull rebase` 
    - [x] dry-run
    - [x] set-upstream
- 🔶 Fetch 

---
## Installation

- Windows: Download the latest .exe from the releases page.
    - `grp.exe` is the portable version
    - `grp-0.7.1-x86_64.msi` is the installer
    - Or use winget.
    ~~~bash
    winget install --id feraxhp.grp
    ~~~
- Ubuntu: See releases page for the latest deb package.
```bash
wget -O paquete.deb <URL_DEL_PAQUETE> 
dpkg sudo dpkg -i paquete.deb 
sudo apt-get install -f
```
- fedora: See releases page for the latest rpm package.
```bash
sudo dnf install <URL_DEL_PAQUETE>
```
- Arch Linux: See build instructions below.
- Other Linux distributions: See build instructions below.
- MacOS: See build instructions below.

---
## Completions

### bash
~~~bash
source <(COMPLETE=bash grp)
~~~

### zsh
~~~zsh
source <(COMPLETE=zsh grp)
~~~

### fish
~~~fish
COMPLETE=fish ./target/debug/grp | source
~~~

---
## Configuration

The grp configurations are store in json. the first time you run
grp, it will create the configurations file with the base config.

if you prefer a gide configuration use: `grp config add`

the location of the config folder depends on the platform
- linux: `$HOME/.config/girep/config.json`
- windows `%appdata%/girep/config.json`
- mac: `$HOME/Library/Application Support/girep/config.json`

the basic structure looks like this:
```json
{
  "default": "",
  "pconf": []
}
```

grp manage the platforms in objets called pcofs. in every pconf you have to add

- **name**: Is the name for the pconf, it is used to determine the platform. 
- **owner**: Is the username that will use by default to request in the platform.
- **token**: Is a user generated token used to authenticate the request.
- **type**: type of the platform. currently allows `github`, `gitea` and `gitlab`.
- **endpoint**: the endpoint to make the request 
  - examples:
    - `"api.github.com"`: for GitHub.
    - `"gitea.com"`: for Gitea.
    - `"tea.example.com"`: for Gitea on custom host.
    - `localhost:3244`: for gitea on localhost.

here is an example for a complete config file:

```json
{
  "default": "gh",
  "pconf": [
      {
      "name": "gh",
      "owner": "feraxhp",
      "token": "<token generated>",
      "type": "github",
      "endpoint": "api.github.com"
    },
    {
      "name": "tea",
      "owner": "feraxhp",
      "token": "<token generated>",
      "type": "gitea",
      "endpoint": "tea.example.com"
    },
    {
      "name": "glab",
      "owner": "feraxhp",
      "token": "<token generated>",
      "type": "gitlab",
      "endpoint": "gitlab.example.com"
    }
  ]
}
```

---
## Build and Run

### Dependencies

Make sure you have the following dependencies installed on your system:

- #### [dependencies](dependencies.md)
- Cargo (Rust)

### Build
```bash

# clone the repository
git clone https://github.com/feraxhp/grp.git
cd grp

# if you want to install it on your system
cargo install --path .
```

---
`*` Jetbrains has removed access for `Jetbrains space` So, is removed for the planed support.

`1` Github does not allow create orgs by the API for security reasons

`2` Some GitLab admins not allow create groups by the API for security reasons, however for sub-groups yes.

`3` Delete for GitLab has an aditional step, so if you whant to just mark for delition something you have to add the flag `--soft`

---
## Need more Functionality?

If you need more functionality, feel free to open an issue or a pull request.
remember to follow the [contribution guidelines](CONTRIBUTING.md)
