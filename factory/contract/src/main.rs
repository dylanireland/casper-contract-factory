#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;
use alloc::vec;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, CLType, EntryPointAccess, EntryPointType, EntryPoints, EntryPoint, Parameter, contracts::NamedKeys};
use crate::alloc::string::ToString;

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    UserError = 0
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn generate() {

    let ur = storage::new_uref("test");
    
    let mut named_keys = NamedKeys::new();
    named_keys.insert(String::from("test"), ur.into());

    let (stored_contract_hash, _contract_version) = storage::new_contract(
        EntryPoints::new(),
        Some(named_keys),
        Some("generated_contract_package".to_string()),
        Some("generated_contract_access_uref".to_string()),
    );

    runtime::put_key("generated_contract", stored_contract_hash.into());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "generate",
        vec![
            Parameter::new("purse", CLType::URef),
            //Parameter::new("caller", CLType::Key)
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (stored_contract_hash, _contract_version) = storage::new_contract(
        entry_points,
        None,
        Some("factory_contract_package".to_string()),
        Some("factory_contract_access_uref".to_string()),
    );

    runtime::put_key("factory_contract", stored_contract_hash.into());
}
