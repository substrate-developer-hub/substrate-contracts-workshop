Transferring Tokens
===

So at this point, we have a single user that owns all the tokens for the contract. However, it's not
_really_ a useful token unless you can transfer them to other people...

Let's do that!

## Transfer Functions

The `transfer` function does exactly what you might expect: it allows the user calling the contract
to transfer some funds they own to another user.

You will notice in our template code there is a public function `transfer` and an internal function
`transfer_from_to`. We have done this because in the future, we will be reusing the logic for a
token transfer when we enable third party allowances and spending on-behalf-of.

### transfer_from_to()

```rust
fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
  /* --snip-- */
}
```

The `transfer_from_to` function will be built without any _authorization_ checks. Because it is an
internal function we fully control when it gets called. However, it will have all logical checks
around managing the balances between accounts.

Really we just need to check for one thing: make sure that the `from` account has enough funds to
send to the `to` account, something likes the following:

```rust
if balance_from < value {
    return false
}
```

Remember that the `transfer` function and other public functions return a bool to indicate success.
If the `from` account does not have enough balance to satisfy the transfer, we will exit early and
return false, not making any changes to the contract state. Our `transfer_from_to` will simply
forward the "success" `bool` up to the function that calls it.

### transfer()

```rust
#[ink(message)]
pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
  /* --snip-- */
}
```

Finally, the `transfer` function will simply call into the `transfer_from_to` with the `from`
parameter automatically set to the `self.env().caller()`. This is our "authorization check" since
the contract caller is always authorized to move their own funds.

## Transfer Math

There really is not much to say about the simple math executed within a token transfer.

1. First we get the current balance of both the `from` and `to` account, making sure to use our
`balance_of_or_zero()` getter.
2. Then we make the logic check mentioned above to ensure the `from` balance has enough funds to
send `value`.
3. Finally, we subtract that `value` from the `from` balance and add it to the `to` balance and
insert those new values back in.

## Your Turn

Follow the `ACTION`s in the template code to build your transfer function.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/2.2-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.2-finished-code.rs ':include :type=code embed-final')

#### ** Previous Solution **

[embedded-code-previous](./assets/2.1-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
