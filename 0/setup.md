Setup
===

To follow this tutorial, you will need to set up some stuff on your computer.

## Substrate Prerequisites

To get started, you need to make sure your computer is set up to build Substrate.

If you are using __OSX__ or most popular __Linux__ distros, you can do it by running:

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

```
rustup target add wasm32-unknown-unknown --toolchain stable
rustup component add rust-src --toolchain nightly
rustup toolchain install nightly-2020-06-01
rustup target add wasm32-unknown-unknown --toolchain nightly-2020-06-01
```
If you are using another operating system, like __Windows__, follow the [installation instructions](https://substrate.dev/docs/en/knowledgebase/getting-started/windows-users) on the Substrate Developer Hub.

## Installing A Substrate Node

We need to use a Substrate node with the built-in Contracts module. For this workshop we'll use the pre-designed Substrate node client.
```bash
cargo +nightly-2020-06-01 install node-cli --git https://github.com/paritytech/substrate.git --tag v2.0.0-rc4 --force --locked
```

## ink! CLI

The final tool we will be installing is the ink! command line utility which will make setting up Substrate smart contract projects easier.

You can install the utility using Cargo with:

```bash
cargo install cargo-contract --vers 0.6.2 --force
```

You can then use `cargo contract --help` to start exploring the commands made available to you.
> **Note:** The ink! CLI is under heavy development and some of its commands are not implemented, yet!
