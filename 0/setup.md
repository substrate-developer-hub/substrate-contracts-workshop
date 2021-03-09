Setup
===

To follow this tutorial, you will need to set up some stuff on your computer.

## Substrate Prerequisites

Follow the
[official installation steps](https://substrate.dev/docs/en/knowledgebase/getting-started/) from the
Substrate Developer Hub Knowledge Base.

```
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain stable
```

## Installing The Canvas Node

We need to use a Canvas node with the built-in Contracts module. For this workshop we'll use the pre-designed Substrate node client.

```bash
cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --tag v0.1.4 --force --locked
```

## ink! CLI

The final tool we will be installing is the ink! command line utility which will make setting up Substrate smart contract projects easier.

As a pre-requisite for the tool you need to install the [binaryen](https://github.com/WebAssembly/binaryen) package, which is used to optimize the WebAssembly bytecode of the contract.

Many package managers have it available nowadays ‒ e.g. it's a package for [Debian/Ubuntu](https://tracker.debian.org/pkg/binaryen),
[Homebrew](https://formulae.brew.sh/formula/binaryen) and [Arch Linux](https://archlinux.org/packages/community/x86_64/binaryen/).

After you've installed the package execute:

```bash
cargo install cargo-contract --vers 0.10.0 --force --locked
```

You can then use `cargo contract --help` to start exploring the commands made available to you.
