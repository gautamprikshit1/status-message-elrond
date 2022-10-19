#![no_std]

elrond_wasm::imports!();
use elrond_wasm::types::heap::String;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::contract]
pub trait EmptyContract {
    #[view(getMessage)]
    #[storage_mapper("statusMessage")]
    fn status(&self) -> SingleValueMapper<String>;

    #[init]
    fn init(&self) {}

    #[endpoint]
    fn set_status(&self, message: String) {
        self.status().set(&message);
    }
}
