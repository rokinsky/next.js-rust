# Next.js with WebAssembly boilerplate based on Rust

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

Compile Rust code run:
```bash
yarn rust:build
```

Add package locally:
```bash
yarn add wasm@file:wasm/pkg
```

Install other dependencies:
```bash
yarn install
```

Run dev server:
```bash
yarn next:dev
```
