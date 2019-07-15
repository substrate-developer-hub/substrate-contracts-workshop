#![cfg_attr(not(any(test, feature = "test-env")), no_std)]

use ink_core::{
    env::DefaultSrmlTypes,
    memory::format,
    storage,
};
use ink_lang::contract;

contract! {
    #![env = DefaultSrmlTypes]

    // Event emitted when a token transfer occurs
    event Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    }

    // Event emitted when an approval occurs
    // ACTION: Create an `Approval` event with:
    //         * owner: AccountId
    //         * spender: AccountId
    //         * value: Balance

    /// The storage items for a typical ERC20 token implementation.
    struct Erc20 {
        /// The total supply.
        total_supply: storage::Value<Balance>,
        /// The balance of each user.
        balances: storage::HashMap<AccountId, Balance>,
        /// Balances that are spendable by non-owners: (owner, spender) -> allowed
        // ACTION: Create a new `allowances` HashMap which maps
        //         a tuple `(AccountId, AccountId)` to `Balance`
    }

    impl Deploy for Erc20 {
        fn deploy(&mut self, init_value: Balance) {
            self.total_supply.set(init_value);
            self.balances.insert(env.caller(), init_value);
            env.emit(Transfer { 
                from: None,
                to: Some(env.caller()),
                value: init_value
            });
        }
    }

    impl Erc20 {
        /// Returns the total number of tokens in existence.
        pub(external) fn total_supply(&self) -> Balance {
            let total_supply = *self.total_supply;
            env.println(&format!("Erc20::total_supply = {:?}", total_supply));
            total_supply
        }

        /// Returns the balance of the given AccountId.
        pub(external) fn balance_of(&self, owner: AccountId) -> Balance {
            let balance = self.balance_of_or_zero(&owner);
            env.println(&format!("Erc20::balance_of(owner = {:?}) = {:?}", owner, balance));
            balance
        }

        /// Returns the amount of tokens that an owner allowed to a spender.
        pub(external) fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            // ACTION: Create a getter for the `allowances` HashMap
            //   HINT: Take a look at the getters above if you forget the details
            // ACTION: Return the `allowance` at the end
        }

        /// Transfers token from the sender to the `to` AccountId.
        pub(external) fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            self.transfer_impl(env, env.caller(), to, value)
        }

        /// Approve the passed AccountId to spend the specified amount of tokens
        /// on the behalf of the message's sender.
        pub(external) fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            // ACTION: Get the `env.caller()` and store it as the `owner`
            // ACTION: Insert the new allowance into the `allowances` HashMap
            //   HINT: The key tuple is `(owner, spender)`
            // ACTION: `emit` the `Approval` event you created using these values
            // ACTION: Return true if everything was successful
        }

        /// Transfer tokens from one AccountId to another.
        pub(external) fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            // ACTION: Get the allowance for `(from, env.caller())` using `allowance_or_zero`
            // ACTION: `if` the `allowance` is less than the `value`, exit early and return `false`
            // ACTION: `insert` the new allowance into the map for `(from, env.caller())`
            // ACTION: Finally, call the `transfer_impl` for `from` and `to`
        }
    }

    impl Erc20 {
        /// Returns the balance of the AccountId or 0 if there is no balance.
        fn balance_of_or_zero(&self, of: &AccountId) -> Balance {
            *self.balances.get(of).unwrap_or(&0)
        }

        /// Returns the allowance or 0 of there is no allowance.
        fn allowance_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            // ACTION: Return the allowance between `(owner, spender)`, `unwrap_or` return 0
        }

        /// Transfers token from a specified AccountId to another AccountId.
        fn transfer_impl(&mut self, env: &mut ink_model::EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes>>, from: AccountId, to: AccountId, value: Balance) -> bool {
            let balance_from = self.balance_of_or_zero(&from);
            let balance_to = self.balance_of_or_zero(&to);
            if balance_from < value {
                return false
            }
            self.balances.insert(from, balance_from - value);
            self.balances.insert(to, balance_to + value);
            env.emit(Transfer { 
                from: Some(from),
                to: Some(to),
                value: value
            });
            true
        }
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;
    use ink_core::env;
    type Types = ink_core::env::DefaultSrmlTypes;

    #[test]
    fn deployment_works() {
        let alice = AccountId::from([0x0; 32]);
        env::test::set_caller::<Types>(alice);

        // Deploy the contract with some `init_value`
        let erc20 = Erc20::deploy_mock(1234);
        // Check that the `total_supply` is `init_value`
        assert_eq!(erc20.total_supply(), 1234);
        // Check that `balance_of` Alice is `init_value`
        assert_eq!(erc20.balance_of(alice), 1234);
    }

    #[test]
    fn transfer_works() {
        let alice = AccountId::from([0x0; 32]);
        let bob = AccountId::from([0x1; 32]);

        env::test::set_caller::<Types>(alice);
        // Deploy the contract with some `init_value`
        let mut erc20 = Erc20::deploy_mock(1234);
        // Alice does not have enough funds for this
        assert_eq!(erc20.transfer(bob, 4321), false);
        // Alice can do this though
        assert_eq!(erc20.transfer(bob, 234), true);
        // Check Alice and Bob have the expected balance
        assert_eq!(erc20.balance_of(alice), 1000);
        assert_eq!(erc20.balance_of(bob), 234);
    }

    #[test]
    fn allowance_works() {
        let alice = AccountId::from([0x0; 32]);
        let bob = AccountId::from([0x1; 32]);
        let charlie = AccountId::from([0x2; 32]);

        env::test::set_caller::<Types>(alice);
        // Deploy the contract with some `init_value`
        let mut erc20 = Erc20::deploy_mock(1234);
        // Bob does not have an allowance from Alice's balance
        assert_eq!(erc20.allowance(alice, bob), 0);
        // Thus, Bob cannot transfer out of Alice's account
        env::test::set_caller::<Types>(bob);
        assert_eq!(erc20.transfer_from(alice, bob, 1), false);
        // Alice can approve bob for some of her funds
        env::test::set_caller::<Types>(alice);
        assert_eq!(erc20.approve(bob, 20), true);
        // And the allowance reflects that correctly
        assert_eq!(erc20.allowance(alice, bob), 20);

        // Charlie cannot send on behalf of Bob
        env::test::set_caller::<Types>(charlie);
        assert_eq!(erc20.transfer_from(alice, bob, 10), false);
        // Bob cannot transfer more than he is allowed
        env::test::set_caller::<Types>(bob);
        assert_eq!(erc20.transfer_from(alice, charlie, 25), false);
        // A smaller amount should work though
        assert_eq!(erc20.transfer_from(alice, charlie, 10), true);
        // Check that the allowance is updated
        assert_eq!(erc20.allowance(alice, bob), 10);
        // and the balance transferred to the right person
        assert_eq!(erc20.balance_of(charlie), 10);
    }

    #[test]
    fn events_work() {
        let alice = AccountId::from([0x0; 32]);
        let bob = AccountId::from([0x1; 32]);

        // No events to start
        env::test::set_caller::<Types>(alice);
        assert_eq!(env::test::emitted_events::<Types>().count(), 0);
        // Event should be emitted for initial minting
        let mut erc20 = Erc20::deploy_mock(1234);
        assert_eq!(env::test::emitted_events::<Types>().count(), 1);
        // Event should be emitted for transfers
        assert_eq!(erc20.transfer(bob, 10), true);
        assert_eq!(env::test::emitted_events::<Types>().count(), 2);
        // Event should be emitted for approvals
        assert_eq!(erc20.approve(bob, 20), true);
        assert_eq!(env::test::emitted_events::<Types>().count(), 3);
    }
}
