Supporting Approvals and Transfer From
===

We are almost there! Our token contract can now transfer funds from user to user and tell the
outside world what is going on when this happens. All that is left to do is introduce the `approve`
and `transfer_from` functions.

## Third Party Transfers

This section is all about adding the ability for other accounts to safely spend some amount of your
tokens.

The immediate question should be: "Why the heck would I want that?"

Well, one such scenario is to support Decentralized Exchanges. Basically, other smart contracts can
allow you to exchange tokens with other users, usually one type of token for another. However, these
"bids" do not always execute right away. Maybe you want to get a really good deal for token trade,
and will hold out until that trade is met.

Well, rather than giving your tokens directly to the contract (an escrow), you can simply "approve"
them to spend some of your tokens on your behalf! This means that during the time while you are
waiting for a trade to execute, you can still control and spend your funds if needed. Better yet,
you can approve multiple different contracts or users to access your funds, so if one contract
offers the best trade, you do not need to pull out funds from the other and move them, a sometimes
costly and time consuming process.

So hopefully you can see why a feature like this would be useful, but how can we do it safely?

We use a two step process: **Approve** and **Transfer From**.

### Approve

Approving another account to spend your funds is the first step in the third party transfer process.
A token owner can specify another account and any arbitrary number of tokens it can spend on the
owner's behalf. The owner need not have all their funds approved to be spent by others; in the
situation where there is not enough funds, the approved account can spend up to the approved amount
from the owner's balance.

When an account calls `approve` multiple times, the approved value simply overwrites any existing
value that was approved in the past. By default, the approved value between any two accounts is `0`,
and a user can always call approve for `0` to revoke access to their funds from another account.

To store approvals in our contract, we need to use a slightly fancy `HashMap`.

Since each account can have a different amount approved for any other accounts to use, we need to
use a tuple as our key which simply points to a balance value. Here is an example of what that
would look like:

```rust
pub struct Erc20 {
    /// Balances that are spendable by non-owners: (owner, spender) -> allowed
    allowances: ink_storage::collections::HashMap<(AccountId, AccountId), Balance>,
}
```

Here we have defined the tuple to represent `(owner, spender)` such that we can look up how much a
"spender" can spend from an "owner's" balance using the `AccountId`s in this tuple. Remember that we
will need to again create an `allowance_of_or_zero` function to help us get the allowance of an
account when it is not initialized, and a getter function called `allowance` to look up the current
value for any pair of accounts.

```rust
/// Approve the passed AccountId to spend the specified amount of tokens
/// on the behalf of the message's sender.
#[ink(message)]
pub fn approve(&mut self, spender: AccountId, value: Balance) -> bool {/* --snip-- */}
```

When you call the `approve` function, you simply insert the `value` specified into storage. The `owner` is always the `self.env().caller()`, ensuring that the function call is always authorized.

### Transfer From

Finally, once we have set up an approval for one account to spend on-behalf-of another, we need to create a special `transfer_from` function which enables an approved user to transfer those funds.

As mentioned earlier, we will take advantage of the private `transfer_from_to` function to do the bulk of our transfer logic. All we need to introduce is the _authorization_ logic again.

So what does it mean to be authorized to call this function?

1. The `self.env().caller()` must have some allowance to spend funds from the `from` account.
2. The allowance must not be less than the value trying to be transferred.

In code, that can easily be represented like so:

```rust
let allowance = self.allowance_of_or_zero(&from, &self.env().caller());
if allowance < value {
    return false
}
/* --snip-- */
true
```

Again, we exit early and return false if our authorization does not pass.

If everything looks good though, we simply `insert` the updated allowance into the `allowance` HashMap (`let new_allowance = allowance - value`), and call the `transfer_from_to` between the specified `from` and `to` accounts.

## Be Careful!

If you glaze over the logic of this function too quickly, you may introduce a bug into your smart contract. Remember when calling `transfer_from`, the `self.env().caller()` and the `from` account is used to look up the current allowance, but the `transfer_from` function is called between the `from` and `to` account specified.

There are three account variables in play whenever `transfer_from` is called, and you need to make sure to use them correctly! Hopefully our test will catch any mistake you make.

## Your Turn!

You are almost there! This is the last piece of the ERC20 token contract.

Follow the `ACTION`s in the contract template to finish your ERC20 implementation.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/2.4-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.4-finished-code.rs ':include :type=code embed-final')

#### ** Previous Solution **

[embedded-code-previous](./assets/2.3-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->
