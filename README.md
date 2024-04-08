# Playground for "rust_serde_json"

> try out rust serde json convert to/from rust struct

- [env](#env)
- [init](#init)

## Env

> Try out on this os

```bash
cat /etc/os-release 
PRETTY_NAME="Ubuntu 22.04.4 LTS"
NAME="Ubuntu"
VERSION_ID="22.04"
VERSION="22.04.4 LTS (Jammy Jellyfish)"
VERSION_CODENAME=jammy
ID=ubuntu
ID_LIKE=debian
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
UBUNTU_CODENAME=jammy
```

## Init

## Setup

> Shell command for setup project

```bash
set --exitonerror
# set -e
export PROJECT_NAME="rust_serde_json"
clear
rm -f -R ./${PROJECT_NAME}
mkdir ./${PROJECT_NAME} && cd $_
cargo init .
cargo add serde
cargo add serde-json
cargo install cargo-make
cargo update
cargo upgrade
cargo build
mkdir examples
cp ./src/main.rs ./examples/example.rs
sed -i 's/world/example/g' ./examples/example.rs
cargo build --example example
cargo run --example example
touch README.md
ln -s README.md README
touch FROM_HERE.md
mkdir examples
cat ./Cargo.toml
tree .
history -w /dev/stdin
```

```bash
cat Cargo.toml 
[package]
name = "rust_serde_json"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0.197"
serde_json = "1.0.115"
```
