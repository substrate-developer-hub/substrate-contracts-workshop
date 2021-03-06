Introduction
===

In this chapter, we will show you how you can build an ERC20 token contract with ink!.

Over the course of this chapter, we will cover:

- Initial token minting
- Tokens transfer
- Approvals and third party transfers
- Emitting runtime events through Substrate

But first, we will go over the ERC20 standard for those of you who are not familiar.

## ERC20 Standard

The [ERC20 token standard](https://eips.ethereum.org/EIPS/eip-20) defines the interface for the most popular Ethereum smart contract.

```javascript
// ----------------------------------------------------------------------------
// ERC Token Standard #20 Interface
// https://github.com/ethereum/EIPs/blob/master/EIPS/eip-20.md
// ----------------------------------------------------------------------------

contract ERC20Interface {
    // Storage Getters
    function totalSupply() public view returns (uint);
    function balanceOf(address tokenOwner) public view returns (uint balance);
    function allowance(address tokenOwner, address spender) public view returns (uint remaining);

    // Public Functions
    function transfer(address to, uint tokens) public returns (bool success);
    function approve(address spender, uint tokens) public returns (bool success);
    function transferFrom(address from, address to, uint tokens) public returns (bool success);

    // Contract Events
    event Transfer(address indexed from, address indexed to, uint tokens);
    event Approval(address indexed tokenOwner, address indexed spender, uint tokens);
}
```

In summary, it allows individuals to deploy their own cryptocurrency on top of an existing smart
contract platform. There isn't much magic happening in this contract. Users balances are stored in a
HashMap, and a set of APIs are built to allow users to transfer tokens they own or allow a third
party to transfer some amount of tokens on their behalf. Most importantly, all of this logic is
implemented ensuring that funds are not unintentionally created or destroyed, and that a user's
funds are protected from malicious actors.

Note that all the public functions return a `bool` which specifies if the call was successful or not.
We will adhere to that specification.
