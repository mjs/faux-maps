# faux-maps

Generates cute, random maps in the browser using WebAssembly. Most of the heavy
lifting is done by the excellent [Noise](https://crates.io/crates/noise) crate.

## How to install dependencies

```sh
npm install
```

## How to run in debug mode

Need this workaround to avoid TLS issues (the Nix flake devshell already has this):
```
export NODE_OPTIONS=--openssl-legacy-provider
```

This builds the project and opens it in a new browser tab. Auto-reloads when
the project changes.

```sh
npm start
```

## How to build a release

This builds the project and places it into the `dist` folder.

```sh
npm run build
```
