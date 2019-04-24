Building Your Contract
===

The ink! CLI also generates a build script called `build.sh`:

```bash
#!/bin/bash

PROJNAME=flipper

CARGO_INCREMENTAL=0 &&
cargo +nightly build --release --features generate-api-description --target=wasm32-unknown-unknown --verbose &&
wasm2wat -o target/$PROJNAME.wat target/wasm32-unknown-unknown/release/$PROJNAME.wasm &&
cat target/$PROJNAME.wat | sed "s/(import \"env\" \"memory\" (memory (;0;) 2))/(import \"env\" \"memory\" (memory (;0;) 2 16))/" > target/$PROJNAME-fixed.wat &&
wat2wasm -o target/$PROJNAME.wasm target/$PROJNAME-fixed.wat &&
wasm-opt -Oz target/$PROJNAME.wasm -o target/$PROJNAME-opt.wasm &&
wasm-prune --exports call,deploy target/$PROJNAME-opt.wasm target/$PROJNAME-pruned.wasm
```

This file will be used to compile your contract source code to WASM. You can see that it depends on the Wasm utilities we installed earlier. You can find out more about each command in the Terminology section below.

To compile the smart contract, we need to make the build script executable with `chmod` and then we can run it:

```bash
chmod +x build.sh
./build.sh
```

If all goes well, you should see a `target` folder being created with 5 relevant files corresponding to the steps in the script:

```
flipper.wat
flipper-fixed.wat
flipper.wasm
flipper-opt.wasm
flipper-pruned.wasm
```

The final, optimized `flipper-pruned.wasm` file is what we will actually deploy to our substrate chain.

## Contract ABI

You will also notice a JSON file which is generated during the build script:

```
Flipper.json
```

This is your contract's application binary interface (ABI). Let's take a look inside:

```json
{
    "name": "Flipper",
    "deploy": {
        "args": []
    },
    "messages": [
        {
            "name": "flip",
            "selector": 970692492,
            "mutates": true,
            "args": [],
            "return_type": null
        },
        {
            "name": "get",
            "selector": 4266279973,
            "mutates": false,
            "args": [],
            "return_type": "bool"
        }
    ]
}
```

You can see that this file describes the interface that can be used to interact with your contract.

If there are any deployment variables needed when instantiating a new contract, those will be defined in the `deploy` section. All the public functions your contract exposes can be found in `messages` along with its function name, function parameters, return type, and whether the function is read-only.

There is also a `selector` which is a hash of the function name and is used to route your contract calls to the correct function.

The Polkadot UI uses this file to generate a friendly interface for deploying and interacting with your contract. :)

---

**Learn More**

One line in the build script we should call out is:

```bash
cat target/$PROJNAME.wat | sed "s/(import \"env\" \"memory\" (memory (;0;) 2))/(import \"env\" \"memory\" (memory (;0;) 2 16))/" > target/$PROJNAME-fixed.wat &&
```

TL;DR, this line is adding a maximum size to the Wasm memory declaration, which by default is not included.

WebAssembly modules can use two parameters to specify how much memory it wants:

1. Initial Size - the size of the memory when it is first imported.
2. Maximum Size - the maximum size the memory can grow to.

It is encoded like:

```
(import "env" "memory" (memory <initial> <maximum>))
```

Maximum can be absent in this case it is implicitly set to 4GB.

---

**Terminology**

- [.wasm binary file format](http://webassembly.github.io/spec/core/binary/index.html) - WebAssembly (WASM) format of binary WASM modules

- [.wat text file format](http://webassembly.github.io/spec/core/text/index.html) - WebAssembly Text Format (WAT) is similar to the binary (.wasm) format, but designed to be human readable.

- [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) - Compile the library target of the selected Rust packages.

- [`wasm-opt`](https://github.com/WebAssembly/binaryen#tools) - Tool from the binaryen project that when run on .wasm format binary input file will transform, optimize, and/or run instrumentation passes on it and create .wasm binaries that are usually both smaller and execute faster.

- [`wasm-prune`](https://github.com/paritytech/wasm-utils#symbols-pruning-wasm-prune) - Optimize the WASM symbols tree of a .wasm format binary input file and only leave those elements that are used by contract call function entry.

- [`wasm32-unknown-unknown`](https://rustwasm.github.io/docs/wasm-bindgen/reference/rust-targets.html#supported-rust-targets) - Compile dynamic Rust libraries using a LLVM WASM backend directly to generate a "bare bones" small and fast binary WASM module (.wasm).

- [`wasm2wat`](https://github.com/WebAssembly/wabt#wabt-the-webassembly-binary-toolkit) - It translates from the binary WASM module (.wasm) format back to the WASM text (.wat) format. It is a command from the WebAssembly Binary Toolkit (WABT) that performs the inverse of `wat2wasm`.

- [`wat2wasm`](https://github.com/WebAssembly/wabt#wabt-the-webassembly-binary-toolkit) - Translates from the WASM text (.wat) format to the binary WASM module (.wasm) format. It is a command from the WebAssembly Binary Toolkit (WABT) that performs the inverse of `wasm2wat`.

---