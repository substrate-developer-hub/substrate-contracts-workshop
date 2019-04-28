Creating the ERC-20 Template
===

We are going to start another ink! project for this new ERC-20 token contract we will build.

Back in your working directory, run:

```bash
cargo contract new erc20
```

Again, we will replace the `src/lib.rs` file content with the template provided on this page.

You will notice that the template for the ERC-20 token is VERY similar to the Incrementer contract. (Coincidence? ¯\\_(ツ)_/¯)

The storage (so far) consists of:

- A storage Value: representing the total supply of tokens in our contract.
- A storage HashMap: representing the individual balance of each account.

## ERC-20 Deployment

The most basic ERC-20 token contract is a fixed supply token. During contract deployment, all the tokens will be automatically given to the contract creator. It is then up to the user to distribute those tokens to other users as they see fit.

Of course, this is not the only way to mint and distribute tokens, but the most simple one and what we will be doing here.

So remember to `set` the total balance and `insert` the balance of the `env.caller()`

## Your Turn!

This chapter should be nothing more than a quick refresher of the content you already learned.

You need to:

- Set up a deployment function which initializes the two storage items
- Create getters for both storage items
- Create a `balance_of_or_zero` function to handle reading values from the HashMap

Remember to run `cargo test --features test-env` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/2.1-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.1-finished-code.rs ':include :type=code embed-final')

<!-- tabs:end -->

**Learn More**

* [Parity Substrate sample runtime implementation of the ERC-20 token standard coded in Rust](https://github.com/parity-samples/substrate-erc20).
* [Parity Substrate sample runtime implementation of a multiple ERC-20 token standard coded in Rust](https://github.com/parity-samples/substrate-erc20-multi).
* [Parity Substrate sample runtime implementation of the ERC-721 non-fungible token (NFT) standard coded in Rust](https://github.com/parity-samples/substrate-erc721)
* [Parity Substrate sample runtime implementation of (a subset of) Token Curated Registries (TCR)](https://github.com/parity-samples/substrate-tcr).
* [Ethereum sample ERC-20 token standard coded in Solidity](https://theethereum.wiki/w/index.php/ERC20_Token_Standard).
* [Ethereum EIP 20: ERC-20 token standard](https://eips.ethereum.org/EIPS/eip-20).
* [Ethereum EIP 721: ERC-721 non-fungible token standard](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-721.md).

---
