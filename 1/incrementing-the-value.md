Incrementing the Value
===

It's time to let our users modify storage!

## Mutable and Immutable Functions

You may have noticed that the function templates included `self` as the first parameter of the contract functions. It is through `self` that you gain access to all your contract functions and storage items.

If you are simply _reading_ from the contract storage, you only need to pass `&self`. But if you want to _modify_ storage items, you will need to explicitly mark it as mutable, `&mut self`.

```rust
impl MyContract {
    #[ink(message)]
    pub fn my_getter(&self) -> u32 {
        self.my_number
    } 

    #[ink(message)]
    pub fn my_setter(&mut self, new_value: u32) {
        self.my_number = new_value;
    }
}
```

## Your Turn

Follow the `ACTION`s in the template code.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/1.4-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/1.4-finished-code.rs ':include :type=code embed-final')

#### ** Previous Solution **

[embedded-code-previous](./assets/1.3-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
