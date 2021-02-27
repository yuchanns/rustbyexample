## yew-tailwindcss
use tailwindcss with yew framework

![](https://github.com/yuchanns/rustbyexample/workflows/yew-tailwindcss/badge.svg?branch=main)

## Preview
[Tencent CDN](https://yew-tailwindcss.yuchanns.xyz/)

### Develop
`cargo-watch` is required: `cargo install cargo-watch`.
```bash
cargo-watch -s "yarn --cwd styles build" \
  -s "wasm-pack build --target web --out-name wasm --out-dir ./dist" \
  -s "yarn --cwd styles serve" 
```

### Build
**Build Tailwindcss**
```bash
yarn --cwd styles build
```
**Build Wasm**
```bash
wasm-pack build --target web --out-name wasm --out-dir ./dist
```