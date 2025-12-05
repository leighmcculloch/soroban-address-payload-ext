#![no_std]
use soroban_sdk::unwrap::UnwrapOptimized;
use soroban_sdk::xdr::ToXdr;
use soroban_sdk::{Address, Bytes, BytesN, Env};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AddressPayloadType {
    AccountEd25519PublicKey,
    ContractHash,
}

pub trait AddressPayloadExt {
    fn payload(&self, env: &Env) -> Option<(AddressPayloadType, Bytes)>;
}

impl AddressPayloadExt for Address {
    fn payload(&self, env: &Env) -> Option<(AddressPayloadType, Bytes)> {
        let xdr = self.to_xdr(env);
        // Skip over ScVal discriminant because we know it is an ScAddress.
        let xdr = xdr.slice(4..);
        // Decode ScAddress
        let addr_type: BytesN<4> = xdr.slice(0..4).try_into().unwrap_optimized();
        match addr_type.to_array() {
            [0, 0, 0, 0] => {
                // Decode PublicKey
                let public_key_type: BytesN<4> = xdr.slice(4..8).try_into().unwrap_optimized();
                match public_key_type.to_array() {
                    [0, 0, 0, 0] => Some((
                        AddressPayloadType::AccountEd25519PublicKey,
                        xdr.slice(8..40),
                    )),
                    _ => None,
                }
            }
            [0, 0, 0, 1] => Some((AddressPayloadType::ContractHash, xdr.slice(4..36))),
            _ => None,
        }
    }
}
