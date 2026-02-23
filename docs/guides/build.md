
## Dependencies

### fedora
- openssl-devel

```bash
sudo dnf install -y @development-tools openssl-devel
```

### ubuntu
- libssl-dev
- pkg-config
- build-essential (pack)

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev
```

### other

Not sure about it, please [contribute]() if you know it.

## Build

### Clone the repository

~~~bash
git clone https://github.com/feraxhp/grp.git && cd grp
~~~

### Install
~~~bash
cargo install --path .
~~~
