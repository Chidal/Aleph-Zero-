
### `src/lib.rs`
```rust
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod mytoken {
    #[ink(storage)]
    pub struct MyToken {
        total_supply: u64,
        balances: ink_storage::collections::HashMap<AccountId, u64>,
    }

    impl MyToken {
        #[ink(constructor)]
        pub fn new(initial_supply: u64) -> Self {
            let mut balances = ink_storage::collections::HashMap::new();
            let caller = Self::env().caller();
            balances.insert(caller, initial_supply);

            Self {
                total_supply: initial_supply,
                balances,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u64 {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u64 {
            *self.balances.get(&account).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: u64) -> bool {
            let caller = Self::env().caller();
            let caller_balance = self.balance_of(caller);

            if caller_balance < value {
                return false;
            }

            self.balances.insert(caller, caller_balance - value);
            let recipient_balance = self.balance_of(to);
            self.balances.insert(to, recipient_balance + value);
            true
        }
    }
}
