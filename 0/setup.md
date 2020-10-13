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
rustup install nightly-2020-10-06
rustup target add wasm32-unknown-unknown --toolchain nightly-2020-10-06
```

## Installing The Canvas Node

We need to use a Canvas node with the built-in Contracts module. For this workshop we'll use the pre-designed Substrate node client.

```bash
cargo +nightly-2020-10-06 install canvas-node --git https://github.com/paritytech/canvas-node.git --tag v0.1.0 --force
```

## ink! CLI

The final tool we will be installing is the ink! command line utility which will make setting up Substrate smart contract projects easier.

You can install the utility using Cargo with:

```bash
cargo install cargo-contract --vers 0.7.0 --force
```

You can then use `cargo contract --help` to start exploring the commands made available to you.
