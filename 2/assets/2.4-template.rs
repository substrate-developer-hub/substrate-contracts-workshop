#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_core::storage;
use ink_lang2 as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    #[ink(storage)]
    struct Erc20 {
        /// The total supply.
        total_supply: storage::Value<Balance>,
        /// The balance of each user.
        balances: storage::HashMap<AccountId, Balance>,
        /// Approved spender on behalf of the message's sender.
        allowances: storage::HashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    struct Transferred {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    struct Approved {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        fn new(&mut self, initial_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(initial_supply);
            self.balances.insert(caller, initial_supply);
            self.env().emit_event(Transferred {
                from: None,
                to: Some(caller),
                amount: initial_supply,
            });
        }

        #[ink(message)]
        fn total_supply(&self) -> Balance {
            *self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, amount: Balance) -> bool {
            // ACTION: Get the `self.env().caller()` and store it as the `owner`
            // ACTION: Insert the new allowance into the `allowances` HashMap
            //   HINT: The key tuple is `(owner, spender)`
            // ACTION: `emit` the `Approved` event you created using these values
            // ACTION: Return true if everything was successful
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            // ACTION: Create a getter for the `allowances` HashMap
            //   HINT: Take a look at the getters above if you forget the details
            // ACTION: Return the `allowance` amount
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: Balance) -> bool {
            // ACTION: Get the allowance for `(from, self.env().caller())` using `allowance_of_or_zero`
            // ACTION: `if` the `allowance` is less than the `value`, exit early and return `false`
            // ACTION: `insert` the new allowance into the map for `(from, self.env().caller())`
            // ACTION: Finally, call the `transfer_from_to` for `from` and `to`
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, amount: Balance) -> bool {
            let from = self.env().caller();
            self.transfer_from_to(from, to, amount)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < amount {
                return false
            }
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(from, from_balance - amount);
            self.balances.insert(to, to_balance + amount);
            self.env().emit_event(Transferred {
                from: Some(from),
                to: Some(to),
                amount,
            });
            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }
    }
}