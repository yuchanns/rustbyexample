# Writing an OS in Rust
[Writing an OS in Rust](https://os.phil-opp.com/)

![](https://github.com/yuchanns/rustbyexample/workflows/writing-an-os-in-rust/badge.svg?branch=main)

## Requirement
**rustup component**
```sh
rustup component add rust-src
rustup component add llvm-tools-preview
```
**qemu**
* MacOS:
```sh
# install qemu
brew install qemu
# on big sur, add the entitlement to the qemu-system-x86_64 binary
codesign -s - --entitlements .config/entitlements.xml --force /usr/local/bin/qemu-system-x86_64
```
**bootimage**
```sh
cargo install bootimage
```
## Run
```sh
cargo run -- -accel hvf -cpu host
```
