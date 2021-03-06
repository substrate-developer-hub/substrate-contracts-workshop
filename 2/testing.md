Testing Our Contract
===

Now let's conclude our ERC20 token implementation by walking through some test cases we have put forward
when you are filling in the code in the previous sections. In fact if you have been following along
on the coding exercise and running `cargo +nightly test` at the end to make sure the output are okay,
you have been running the test cases all along.

## Motivation

In software engineering practice, it is true that writing automatic test cases cannot be emphasized
enough. There are many type of tests one can write and here we are focusing on writing **Unit Tests**.
This means we as developers know the code logic (versus testing assuming not knowing the logic, a.k.a.
[black-box testing](https://en.wikipedia.org/wiki/Black-box_testing)) and we write test cases to
verify a function perform as we expected by giving it certain inputs and verifying it is returning
the result we expect. Along the way we also test for edge cases, e.g. how it would handle empty value,
or value that is out of its expected bound, and test for certain error handling mechanism is executed
or error message is returned.

The benefit of having unit tests written are so that once our program get big or during
future code refactoring, we can run these tests and if we see them pass, we have the confident that
our main program still works.

## Unit Test Structure

Now let's get to walking through some test cases for smart contract. They can be seen at the
bottom section of the code on the right panel.

```rust
#[cfg(test)]
mod tests {
  use super::*;
  use ink_lang as ink;
  // snip...
}
```

`#[cfg(test)]` is specified so the code section immediately below it is run only when `cargo test`
is executed but not in the normally-executed `cargo run`. We shorthand all code logic defined above
with `use super::*`, and alias `ink_lang` as `ink`.

The first test case is just:

```rust
#[ink::test]
fn new_works() {
  let contract = Erc20::new(777);
  assert_eq!(contract.total_supply(), 777);
}
```

Each test case is a normal function definition returning nothing prepended by the `#[ink::test]`
attribute macro. Inside the function, we setup for certain conditions and then assert for certain
results. If the assert statement fail, it will panic and the test will abort with error message.

In the above test case, we just define a new contract with `777` as the total supply, and verify
the total supply is indeed `777`.

Now, let's walk through a more complex one, the last test case.

```rust
#[ink::test]
fn transfer_from_works() {
  let mut contract = Erc20::new(100);
  assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
  contract.approve(AccountId::from([0x1; 32]), 20);
  contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x0; 32]), 10);
  assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
}
```

We first define a contract with `100` total supply. By default in testing, the account calling for
all contract code is `0x00000001`. So here we assert that the contract creator has all the total supply
at the beginning.

Then we approve `0x00000001` to have an allowance of 20 to transfer his own fund.

> **Note**: Notice this line is called with account `0x00000001`, so it may look silly that `0x00000001` has
> to approve himself to use his own fund. In fact if he call `transfer()` directly, he can still
> transfer fund to others. But with `transfer_from()` we know that `approve()` has the logic to set the
> allowance amount so `0x00000001` can call `transfer_from()` later and succeed.

Afterward, we make a `transfer_from()` call, and ensure the destination account has the expected
amount.

You could also refer to the API doc on further usage of
[`assert!()`](https://doc.rust-lang.org/std/macro.assert.html) and
[`assert_eq!()`](https://doc.rust-lang.org/std/macro.assert_eq.html).

Congratulation! You have completed the tutorial and you write your own ERC20 token smart contract
using ink! in Substrate.

## Next Step

- Learn more about smart contract development at:
https://substrate.dev/docs/en/knowledgebase/smart-contracts/

<!-- tabs:start -->

#### ** Template **
[embedded-code](./assets/2.4-finished-code.rs ':include :type=code embed-template')

<!-- tabs:end -->
