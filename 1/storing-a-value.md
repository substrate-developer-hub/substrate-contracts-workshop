Storing a Value
===

Here is how you would store some simple values in storage:

```rust
#[ink(storage)]
struct MyContract {
    // Store a bool
    my_bool: storage::Value<bool>,
    // Store some number
    my_number: storage::Value<u32>,
}
/* --snip-- */
```

## Supported Types

Contract storage like `storage::Value<T>` is allowed to be generic over types that are encodable and decodable with [Parity Codec](https://github.com/paritytech/parity-codec) which includes the most common types such as `bool`, `u{16,32,64,128}`, `i{8,16,32,64,128}`, `String`, tuples, and arrays.  Note that `u8` is [not currently supported](https://github.com/paritytech/parity-codec/issues/47) in `parity_codec`.

ink! also supports Substrate specific types like `AccountId`, `Balance`, and `Hash`. To use some of these non-primitive types, we have to import them from `use ink_core::storage`.

Here is an example of how you would store an `AccountId` and `Balance`:

```rust
// We are importing the default SRML types
use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod mycontract {

    // Our struct will use those default SRML types
    #[ink(storage)]
    struct MyContract {
        // Store some AccountId
        my_account: storage::Value<AccountId>,
        // Store some Balance
        my_balance: storage::Value<Balance>,
    }
    ...
}
```

You can find all the supported Substrate types in the [`core/env/traits.rs` file](https://github.com/paritytech/ink/blob/master/core/src/env/traits.rs).

## Initializing Storage

> **IMPORTANT:** This section is important. Read it twice. Then have tea and read it again.

Before you can interact with any storage items in an ink! contract, **you must make sure they are initialized!** If you do not initialize a storage item and try to access it, your contract call will not succeed and any state changes caused by the transaction will be reverted. (Gas fees will still be charged though!)

For storage values like the ones above, we can set an initial value with:

```rust
self.my_bool.set(false);
self.my_number.set(42);
self.my_account.set(AccountId::from([0x0; 32]));
self.my_balance.set(1337);
```

This can be done anywhere in our contract logic, but most commonly this happens by using the `[ink(constructor)]` attribute.

## Contract Deployment

Every ink! smart contract must have a contructor which is run once when a contract is created. ink! smart contracts can have many different constructors, it is not limited to only one.

```rust
use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod mycontract {
    #[ink(storage)]
    struct MyContract {
        number: storage::Value<u32>,
    }

    impl MyContract {
        #[ink(constructor)]
        fn new(&mut self, init_value: u32) {
            self.number.set(init_value);
        }

        #[ink(constructor)]
        fn default(&mut self) {
            self.new(0)
        }
    /* --snip-- */
    }
}
```

> **Note:** If you are familiar with Solidity, this is similar to the `constructor` function, however in ink!, it is not optional.

## Your Turn!

Follow the `ACTION`s in the template.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.2-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->
