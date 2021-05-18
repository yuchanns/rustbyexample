# Writing an OS in Rust
[Writing an OS in Rust](https://os.phil-opp.com/)

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
## Run
```sh
cargo run -- -accel hvf -cpu host
```