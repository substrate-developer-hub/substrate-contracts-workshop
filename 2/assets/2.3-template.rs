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
    }

    #[ink(event)]
    struct Transferred {
    //  ACTION: Create a `Transferred` event with:
    //          * from: Option<AccountId>
    //          * to: Option<AccountId>
    //          * amount: Balance
    }

    impl Erc20 {
        #[ink(constructor)]
        fn new(&mut self, initial_supply: Balance) {
            let caller = self.env().caller();
            self.total_supply.set(initial_supply);
            self.balances.insert(caller, initial_supply);
            // ACTION: Call `self.env().emit_event` with the `Transferred` event
            //   HINT: Since we use `Option<AccountId>`, you need to wrap accounts in `Some()`
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
            // ACTION: Call `self.env().emit_event` with the `Transferred` event
            //   HINT: Since we use `Option<AccountId>`, you need to wrap accounts in `Some()`
            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

    }
}