# grp (git repository)

girep is a command line tool that helps you manage your git repositories in the cloud, for difrent platforms. 
It is written in Rust, and it is inspired by [gcli](https://github.com/herrhotzenplotz/gcli).

## Features
- New: 🎉
- Supported: ✅
- Planed support: 🟥
- On development: 🔶

### Repositories

| Feature | GitHub | Gitea | GitLab | Jetbrains Space |
|:-------:|:------:|:-----:|:------:|:---------------:|
| List    |   ✅   |   ✅   |   🟥   |       🟥        |
| Create  |   ✅   |   ✅   |   🟥   |       🟥        |
| Delete  |   ✅   |   ✅   |   🟥   |       🟥        |
|  Clone  |   ✅   |   ✅   |   🟥   |       🟥        |
|  Push   |   ✅   |   ✅   |   🟥   |       🟥        |
|  Pull   |   🎉   |   🎉   |   🟥   |       🟥        |

### Organizations
| Feature | GitHub | Gitea | GitLab | Jetbrains Space |
|:-------:|:------:|:-----:|:------:|:---------------:|
| List    |   ✅   |   ✅  |   🟥   |       🟥        |
| Create  |   ✅   |   ✅  |   🟥   |       🟥        |
| Delete  |   ✅   |   ✅  |   🟥   |       🟥        |


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
- **type**: type of the platform. currently allows `github`, `gitea`.
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
    }
  ]
}
```

---
## Installation

- Windows: Download the latest .exe from the releases page. and add it to your PATH.
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
## Need more Functionality?

If you need more functionality, feel free to open an issue or a pull request.
remember to follow the [contribution guidelines](CONTRIBUTING.md)
