# Next.js with WebAssembly boilerplate based on Rust

🦀 Rust + 🕸 WebAssembly + ▲ Next.js = ❤️

## Overview

This code shows how to import WebAssembly files (`.wasm`) and use them inside a React (client-side) and Node (server-side) using Next.js. Here Rust is compiled to WebAssembly and wrapped in a npm package automatically generating `package.json`, `.wasm`, `.js` and `.d.ts` files.

## Requirements

You will need the standard Rust toolchain, including `rustup` `rustc`, and `cargo`:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install also `wasm-pack`:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## How to use

Install other dependencies:
```bash
yarn install
```

### Development

Compile Rust code run:
```bash
yarn rust:dev
```

Run dev server:
```bash
yarn next:dev
```
