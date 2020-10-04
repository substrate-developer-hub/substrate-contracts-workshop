Storing a Value
===

The first thing we are going to do to the contract template is introduce some storage values.

Here is how you would store some simple values in storage:

```rust
#[ink(storage)]
pub struct MyContract {
    // Store a bool
    my_bool: bool,
    // Store some number
    my_number: u32,
}
/* --snip-- */
```

## Supported Types

Contract may store types that are encodable and decodable with [Parity Codec](https://github.com/paritytech/parity-codec) which includes the most common types such as `bool`, `u{8,16,32,64,128}`, `i{8,16,32,64,128}`, `String`, tuples, and arrays.

ink! provides smart contracts Substrate specific types like `AccountId`, `Balance`, and `Hash` as if they were primitive types. Also ink! provides storage types for more elaborate storage interactions through the storage module:

```rust
use ink_storage::collections::{Vec, HashMap, Stash, Bitvec};
```

Here is an example of how you would store an `AccountId` and `Balance`:

```rust
// We are importing the default PALETTE types
use ink_lang as ink;

#[ink::contract]
impl MyContract {

    // Our struct will use those default PALETTE types
    #[ink(storage)]
    pub struct MyContract {
        // Store some AccountId
        my_account: AccountId,
        // Store some Balance
        my_balance: Balance,
    }
    ...
}
```

You can find all the supported Substrate types in [`crates/storage/src/lib.rs`](https://github.com/paritytech/ink/blob/master/crates/storage/src/lib.rs).

## Contract Deployment

Every ink! smart contract must have a constructor which is run once when a contract is created. ink! smart contracts can have multiple constructors:

```rust
use ink_lang as ink;

#[ink::contract]
mod my_contract {
    #[ink(storage)]
    pub struct MyContract {
        number: u32,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self {
                number: init_value,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                number: Default::default(),
            }
        }
    /* --snip-- */
    }
}
```

## Initializing Storage

> **IMPORTANT:** This section is important. Read it twice. Then have tea and read it again.

Before you can interact with any storage items in an ink! contract, **you must make sure they are initialized!** If you do not initialize a storage item and try to access it, your contract call will not succeed and any state changes caused by the transaction will be reverted. (Gas fees will still be charged though!)

Notice that in the example above, the initial value of the item is set in the constructor:

```rust
pub fn new(init_value: u32) -> Self {
    Self {
        number: init_value,
    }
}
```

This can be done anywhere in our contract logic, but most commonly this happens by using the `#[ink(constructor)]` attribute.

## Your Turn!

Follow the `ACTION`s in the template.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.2-finished-code.rs ':include :type=code embed-final')

#### ** Previous Solution **

[embedded-code-previous](./assets/1.1-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
