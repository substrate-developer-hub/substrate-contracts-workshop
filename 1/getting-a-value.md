Getting a Value
===

Now that we have created and initialized a storage value, we are going to start to interact with it!

## Contract Functions

As you see in the contract template, all of your contract functions are part of your contract module.

```rust
impl MyContract {
    // Public and Private functions go here
}
```

## Public and Private Functions

In Rust, you can make as many implementations as you want. As a stylistic choice, we recommend
breaking up your implementation definitions for your private and public functions:

```rust
impl MyContract {
    /// Public function
    #[ink(message)]
    pub fn my_public_function(&self) {
        /* --snip-- */
    }

    /// Private function
    fn my_private_function(&self) {
        /* --snip-- */
    }

    /* --snip-- */
}
```

You can also choose to split things up however is most clear for your project.

Note that all public functions must use the `#[ink(message)]` attribute.

## Getting a Value

We already showed you how to initialize a storage value. Getting the value is just as simple:

```rust
impl MyContract {
    #[ink(message)]
    pub fn my_getter(&self) -> u32 {
        self.number
    }
}
```

In Rust, if the last expression in a function does not have a semicolon, then it will be the return value.

## Your Turn

Follow the `ACTION`s on the code template provided.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.3-finished-code.rs ':include :type=code embed-final')

#### ** Previous Solution **

[embedded-code-previous](./assets/1.2-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
