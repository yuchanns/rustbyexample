# Yew Functional
Use yew functional

![](https://github.com/yuchanns/rustbyexample/workflows/yew-functional/badge.svg?branch=main)

## Preview
[Tencent CDN](https://yew-functional.yuchanns.xyz/)

## Build
`wasm-pack` is required: `cargo install wasm-pack`
```bash
wasm-pack build --target web --out-name wasm --out-dir ./static
```
## Browse
Just simply choose your favor http server.

For example, browsing with [TheWaWaR/simple-http-server](https://github.com/TheWaWaR/simple-http-server):
```bash
cargo install simple-http-server
rehash
simple-http-server --index=index.html static
```