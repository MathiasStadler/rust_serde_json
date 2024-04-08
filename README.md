# Playground for "rust_serde_json"

> try out rust serde json convert to/from rust struct

<!-- [[_TOC_]] -->

- [env](#env)
- [init](#init)
- [setup](#setup)
- [cargo-make](#cargo-make)

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
cargo update
cargo upgrade
cargo build
cargo run
mkdir ./examples
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

## project init Cargo.toml

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

## [cargo-make](<https://docs.rs/crate/cargo-make/0.3.35#installation>>)

```bash
cargo install cargo-make
```

### cargo show installed packages

```bash
cargo install --list
```

## [create makefile](https://stackoverflow.com/questions/2500436/how-does-cat-eof-work-in-bash)

> [FROM HERE](https://sagiegurari.github.io/cargo-make/)

```bash
cat << EoF > ./Makefile.toml
[tasks.format]
install_crate = "rustfmt"
command = "cargo"
args = ["fmt", "--", "--emit=files"]

[tasks.clean]
command = "cargo"
args = ["clean"]

[tasks.build]
command = "cargo"
args = ["build"]
dependencies = ["clean"]

[tasks.test]
command = "cargo"
args = ["test"]
dependencies = ["clean"]

[tasks.my-flow]
dependencies = [
    "format",
    "build",
    "test"
]
EoF
```

## [Parsing JSON Data into a Rust Structure](https://reintech.io/blog/working-with-json-in-rust)

```bash
cat << EoF > ./examples/json_to_struct.rs
use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    vip: bool,
}
fn process_person() -> Result<()> {
    let data = r#"{"name": "John Doe", "age": 30, "vip": true}"#;
    let p: Person = serde_json::from_str(data)?;
    println!("Please call {} at the number {}", p.name, p.age);
    Ok(())
}
EoF
```

## ## [Serializing a Rust Structure into a JSON String](https://reintech.io/blog/working-with-json-in-rust)

```bash
cat << EoF > ./examples/rust_struct_to_json.rs
use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    vip: bool,
}
fn create_person() -> Result<String> {
    let p = Person {
        name: "John Doe".to_string(),
        age: 43,
        vip: true,
    };
    let j = serde_json::to_string(&p)?;
    Ok(j)
}

fn main() {
    let _= create_person();
}
EoF

```

## run all example

### Version 1

```bash
cargo test --examples
```

### Version 2

```bash
cargo run --example 2>&1 | grep -E '^ ' | xargs -n1 cargo run --example
```

### /w verbose

```bash
cargo run --example 2>&1 | grep -E '^ ' | xargs -n1 --verbose cargo run --example
```

### /w echo

```bash
cargo run --example 2>&1 | grep -E '^ ' | xargs -n1 echo cargo run --example
```

### /w multiple command

```bash
cargo run --example 2>&1 | 
grep -E '^ ' | 
xargs -i sh -c 'echo "command cargo run --example {}" ; cargo run --example {};'
```
