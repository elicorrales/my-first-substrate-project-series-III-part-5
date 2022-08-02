#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod hello_world {

  #[ink(storage)]
  pub struct HelloWorld {
  }

  impl HelloWorld {

    #[ink(constructor)]
    pub fn new() -> Self {
        Self{}
    }

    #[ink(message)]
    pub fn sayhello(&self) {

    }
  }
}
