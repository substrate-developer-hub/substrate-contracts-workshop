Creating an Event
===

Recall that contract calls cannot directly return a value to the outside world.  However, often we will want to indicate to the outside world that something has taken place (e.g., a transaction has occurred or a certain state has been reached).  We can alert others that this has occured using an `Event`.

## Declaring Events

An event can communicate an arbitrary amount of data, defined in a similar manner as a `struct`.  Events should be declared using the `#[ink(event)]` attribute.

For example,

```rust
#[ink(event)]
struct Foo {
    #[ink(topic)]
    from: Option<AccountId>,
    #[ink(topic)]
    to: Option<AccountId>,
    value: Balance,
}
```

This `Foo` event will contain three pieces of data - a value of type `Balance` and two Option-wrapped `AccountId` variables indicating the `from` and `to` accounts. For faster access to the event data they can have indexed fields. We can do this by using the `#[ink(topic)]` attribute. 

One way of retrieving data from an Option<T> variable is using the `.unwrap_or()` function.  You may recall using this in the `my_value_or_zero()` and `balance_of_or_zero()` functions in this project and the Incrementer project.  

```rust
// Since a has a value Some(1), the 1 will be "unwrapped" and placed in c
let c = a.unwrap_or(&0);
// Since b has a value None, the default value 0 will be placed in d
let d = b.unwrap_or(&0);
```

Note that there other ways to interact with `Option` variables.  You can find more information in the Rust language docs here: https://doc.rust-lang.org/std/option/enum.Option.html

## Emitting Events

Now that we have defined what data will be contained within the event and how to declare it, it's time to actually emit some events.  We do this by calling `self.env().emit_event` and include an event as the sole argument to the method call.

Remember that since the `from` and `to` fields are Option<AccountId>, we can't just set them to particular values.  Let's assume we want to set an value of 100 for the initial deployer.  This value does not come from any other account, and so the `from` value should be None.

```rust
self.env()
    .emit_event(
        Transfer {
            from: None,
            to: Some(self.env().caller()),
            value: 100,
        });
```

Note that `value` does not need a `Some()`, as the value is not specified to be stored within an `Option()`.

We want to emit a Foo event every time that a transfer takes place.  In the ERC-20 template that we have been working on, this occurs in two places: first, during the `new` call, and second, every time that `transfer_from_to` is called.

For more examples of event definition and emitting, you can see here: [**click**](https://github.com/paritytech/ink/blob/master/examples/lang/events/src/lib.rs)

## Your Turn!

Follow the ACTIONs in the template code to emit a `Transfer` event every time a token transfer occurs.

Remember to run `cargo +nightly test` to test your work.

<!-- tabs:start -->

#### ** Template **

[embedded-code](./assets/2.3-template.rs ':include :type=code embed-template')

#### ** Solution **

[embedded-code-final](./assets/2.3-finished-code.rs ':include :type=code embed-final')

#### ** Previous Solution **

[embedded-code-previous](./assets/2.2-finished-code.rs ':include :type=code embed-previous')

<!-- tabs:end -->