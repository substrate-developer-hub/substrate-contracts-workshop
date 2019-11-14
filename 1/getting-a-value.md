Getting a Value
===

Now that we have created and initialized a storage value, we are going to start to interact with it!

## Contract Functions

As you can see in the contract template, all of your contract functions are part of your contract module.

```rust
mod mycontract {
    // Public and Private functions can go here
}
```

### Public and Private Functions

In Rust, you can make as many implementations as you want. As a stylistic choice, we recommend breaking up your implementation definitions for your private and public functions:

```rust
mod mycontract {
    /// Public function
    #[ink(message)]
    fn my_public_function(&self) {
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

## Storage Value API

Without going into so much detail, storage values are a part of the underlying ink! core layer. In the background, they use a more primitive `cell` type which holds an `Option<T>`. When we try to get the value from storage, we `unwrap` the value, which is why it panics if it is not initialized!

From [core/storage/value.rs](https://github.com/paritytech/ink/blob/master/core/src/storage/value.rs):

```rust
impl<T> Value<T>
where
    T: scale::Codec,
{
    /// Returns an immutable reference to the wrapped value.
    pub fn get(&self) -> &T {
        self.cell.get().unwrap()
    }

    /// Returns a mutable reference to the wrapped value.
    pub fn get_mut(&mut self) -> &mut T {
        self.cell.get_mut().unwrap()
    }

    /// Sets the wrapped value to the given value.
    pub fn set(&mut self, val: T) {
        self.cell.set(val);
    }
}
```

In that same file, you can find the other APIs exposed by storage values, however these three are the most commonly used.

## Getting a Value

We already showed you how to use `set` when we initialized the storage value. Getting the value is just as simple:

```rust
mod mycontract {
    #[ink(message)]
    fn my_getter(&self) -> u32 {
        let number = *self.my_number.get();
        number
    }
}
```

You should take notice that the `get` API returns a _reference_ to the value, so to actually get the value you need to dereference it with an asterisks (`*`). In Rust, if the last expression in a function does not have a semicolon, then it will be the return value.

You can also drop `.get()` to implicitly get the value:

```rust
mod mycontract {
    #[ink(message)]
    fn my_getter(&self) -> u32 {
        *self.my_number
    }
}
```

## Printing to Debug

A getter like the one we wrote above is great when you are working inside the blockchain runtime, but not very good if you are part of the outside world. **Substrate runtime calls do not return a value.** Submitting a transaction is a fire-and-forget action, and typically there is no way of returning a result. The same is true for existing smart contract platforms like Ethereum where contract calls won't return anything.

ink! provides a very helpful debugging tool for getting messages to the outside world. You used it already when calling the Flipper contract:

```rust
mod flipper {
    ...
    /// Returns the current state.
    #[ink(message)]
    fn get(&self) -> bool {
        self.env().println(&format!("Flipper Value: {:?}", *self.value));
        *self.value
    }
}
```

You can see that `env.println` in combination with `format!` can allow you to make verbose, content rich messages which you will be able to access from your Substrate node terminal.

![An image of println in the terminal for Flipper with false](../0/assets/flipper-println-false.png)

You automatically gain access to `env.println` whenever you write an `ink!` contract. To access `format!`, you need to import it from `ink_core`:

```rust
use ink_core::memory::format,
}
```

These are simply debugging utilities, so they will only deploy to `--dev` nodes. If you try to deploy a contract with `env.println` to a production chain, the Contracts module will reject it. However, for learning purposes, we can use them freely.

## Your Turn!

Follow the `ACTION`s on the code template provided.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.3-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->