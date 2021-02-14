# Conway's Game of Life

A Conway's Game of Life written in Rust WASM.

![](https://github.com/yuchanns/rustbyexample/workflows/game-of-life/badge.svg?branch=main)

## Preview
[Tencent CDN](https://game-of-life.yuchanns.xyz/)

## Build
`wasm-pack` is required: `cargo install wasm-pack`

**Build WASM**
```bash
wasm-pack build
```
Then a folder `pkg` produced.

**Build NPM**
```bash
cd www && npm run build
```
Then get dist files in `www/dist`