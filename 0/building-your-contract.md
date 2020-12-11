Building Your Contract
===

Run the following command to compile your smart contract:

```bash
cargo +nightly contract build
```

This special command will turn your ink! project into a Wasm binary and a metadata file, which contains the contract's ABI.
These two files can be used for deploying your contract to your chain. If all goes well, you should see a `target` folder which contains this `.wasm` and `metadata.json` file.

```
target
└── flipper.wasm
└── metadata.json
```

Let's take a look at the structure of the `meatadata.json`:

``` JSON
{
  "registry": {
    "strings": [...],
    "types": [...]
  },
  "storage": {...},
  "contract": {
    "name": ...,
    "constructors": [...],
    "messages": [...],
    "events": [],
    "docs": []
  }
}
```

You can see that this file describes all the interfaces that can be used to interact with your contract.

* Registry provides the **strings** and custom **types** used throughout the rest of the JSON.
* Storage defines all the **storage** items managed by your contract and how to ultimately access them.
* Contract stores information about the callable functions like  **constructors** and **messages** a user can call to interact with your contract. It also has helpful information like the **events** that are emitted by the contract or any **docs**.

If you look close at the constructors and messages, you will also notice a `selector` which is a 4-byte hash of the function name and is used to route your contract calls to the correct functions.

The Canvas UI uses this file to generate a friendly interface for deploying and interacting with your contract. :)

In the next section we will start a Canvas node and configure the Canvas UI to interact with it.
