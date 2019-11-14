Building Your Contract
===

Run the following command to compile your smart contract:

```bash
cargo contract build
```

This special command will turn your ink! project into a Wasm binary which you can deploy to your chain. If all goes well, you should see a `target` folder which contains this `.wasm` file.

```
flipper
|
+-- target
    |
    +-- flipper.wasm
```

## Contract Metadata
By running the next command we'll generate the contract's metadata:
``` bash
cargo contract generate-abi
```

You should have a new JSON file (`abi.json`) in the same target directory.

``` bash
target
├── flipper.wasm
└── abi.json
```

Let's take a look inside:

``` JSON
{
  "registry": {
    "strings": [
      "Flipper",
      /* --snip-- */
      "flip",
      "get",
      "bool"
    ],
    "types": [
      {
       /* --snip-- */
        "id": "bool",
        "def": "builtin"
      },
    /* --snip-- */
    ]
  },
  /* --snip-- */
  "contract": {
    "name": 1,
    "constructors": [
      {
        "name": 13,
        "selector": 0,
        "args": [],
        "docs": [
          "Initializes our state to `false` upon deploying our smart contract."
        ]
      }
    ],
    "messages": [
      {
        "name": 14,
        "selector": 970692492,
        "mutates": true,
        "args": [],
        "return_type": null,
        "docs": [
          "Flips the current state of our smart contract."
        ]
      },
      {
        "name": 15,
        "selector": 4266279973,
        "mutates": false,
        "args": [],
        "return_type": {
          "ty": 3,
          "display_name": [
            16
          ]
        },
        "docs": [
          "Returns the current state."
        ]
      }
    ],
    "events": [],
    "docs": []
  }
}
```

You can see that this file describes the interface that can be used to interact with your contract.

If there are any deployment variables needed when instantiating a new contract, those will be defined in the `constructors` section. All the public functions your contract exposes can be found in `messages` along with its function name, function parameters, return type, and whether the function is read-only.

There is also a `selector` which is a hash of the function name and is used to route your contract calls to the correct function.

The Polkadot UI uses this file to generate a friendly interface for deploying and interacting with your contract. :)

In the next section we will configure the Polkadot UI.

---

**Learn More**

ink! provides a built-in overflow protection enabled on our `Cargo.toml` file. It is __recommended__ to keep it enabled as a security mechanism.
```
[profile.release]
panic = "abort"           <-- Panics shall be treated as aborts: reduces binary size
lto = true                <-- enable link-time-optimization: more efficient codegen
opt-level = "z"           <-- Optimize for small binary output
overflow-checks = true    <-- Arithmetic overflow protection
```
After running all Rust and LLVM optimizations, we apply extra steps to create a more efficient WebAssembly [`wasm`] file.

WebAssembly modules can use two parameters to specify how much memory it wants:

1. Initial Size - the size of the memory when it is first imported.
2. Maximum Size - the maximum size the memory can grow to.

It is encoded like:

```
(import "env" "memory" (memory <initial> <maximum>))
```

If Maximum Size is absent then it is implicitly set to 4GB.

---
