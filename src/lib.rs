//! Extension trait for extracting the 32-byte payload from a Soroban [`Address`].
//!
//! Provides the [`AddressPayloadExt`] trait which adds a [`payload`][AddressPayloadExt::payload]
//! method to [`Address`] for extracting the underlying 32-byte payload.
//!
//! # Payload Types
//!
//! - **Contract addresses** (C...) contain a 32-byte contract hash that uniquely identifies the
//!   contract instance on the network, not to be confused with the contract wasm hash.
//! - **Account addresses** (G...) contain a 32-byte Ed25519 public key that corresponds to the
//!   account's master key, that depending on the configuration of that account may or may not be a
//!   signer of the acccount.
//!
//! This library supports all address types as of Stellar Protocol 24.
//!
//! # Example
//!
//! ```
//! use soroban_sdk::{Address, Env, String};
//! use soroban_address_payload_ext::{AddressPayloadExt, AddressPayloadType};
//!
//! let env = Env::default();
//! let address = String::from_str(&env, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");
//! let address = Address::from_string(&address);
//!
//! if let Some((payload_type, payload)) = address.payload(&env) {
//!     match payload_type {
//!         AddressPayloadType::ContractHash => {
//!             // 32-byte contract hash
//!         }
//!         AddressPayloadType::AccountEd25519PublicKey => {
//!             // 32-byte ed25519 public key
//!         }
//!     }
//! }
//! ```

#![no_std]
use soroban_sdk::unwrap::UnwrapOptimized;
use soroban_sdk::xdr::{FromXdr, ToXdr};
use soroban_sdk::{Address, Bytes, BytesN, Env};

/// The type of payload contained in an [`Address`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AddressPayloadType {
    /// An Ed25519 public key from an account address (G...).
    AccountEd25519PublicKey,
    /// A contract hash from a contract address (C...).
    ContractHash,
}

/// Extension trait for extracting the 32-byte payload from an [`Address`].
pub trait AddressPayloadExt {
    /// Extracts the 32-byte payload from the address.
    ///
    /// Returns the payload type and the raw 32-byte payload:
    /// - For contract addresses (C...), returns [`AddressPayloadType::ContractHash`]
    ///   and the 32-byte contract hash.
    /// - For account addresses (G...), returns [`AddressPayloadType::AccountEd25519PublicKey`]
    ///   and the 32-byte Ed25519 public key.
    ///
    /// Returns `None` if the address type is not recognized. This may occur if
    /// a new address type has been introduced to the network that this version
    /// of this library is not aware of.
    ///
    /// # Example
    ///
    /// ```
    /// use soroban_sdk::{Address, BytesN, Env, String};
    /// use soroban_address_payload_ext::{AddressPayloadExt, AddressPayloadType};
    ///
    /// let env = Env::default();
    ///
    /// // Contract address (C...)
    /// let address = String::from_str(&env, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");
    /// let address = Address::from_string(&address);
    /// let (payload_type, payload) = address.payload(&env).unwrap();
    /// assert_eq!(payload_type, AddressPayloadType::ContractHash);
    /// assert_eq!(payload.len(), 32);
    ///
    /// // Account address (G...)
    /// let address = String::from_str(&env, "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ");
    /// let address = Address::from_string(&address);
    /// let (payload_type, payload) = address.payload(&env).unwrap();
    /// assert_eq!(payload_type, AddressPayloadType::AccountEd25519PublicKey);
    /// assert_eq!(payload.len(), 32);
    /// ```
    fn payload(&self, env: &Env) -> Option<(AddressPayloadType, Bytes)>;

    /// Constructs an [`Address`] from a payload type and 32-byte payload.
    ///
    /// This is the inverse of [`payload`][AddressPayloadExt::payload].
    ///
    /// # Example
    ///
    /// ```
    /// use soroban_sdk::{Address, Bytes, Env, String, bytes};
    /// use soroban_address_payload_ext::{AddressPayloadExt, AddressPayloadType};
    ///
    /// let env = Env::default();
    ///
    /// // Create a contract address from a 32-byte hash
    /// let hash = bytes!(
    ///     &env,
    ///     0xd7928b72c2703ccfeaf7eb9ff4ef4d504a55a8b979fc9b450ea2c842b4d1ce61
    /// );
    /// let address = Address::from_payload(&env, AddressPayloadType::ContractHash, &hash);
    /// assert_eq!(
    ///     address.to_string().to_string(),
    ///     "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"
    /// );
    /// ```
    fn from_payload(env: &Env, payload_type: AddressPayloadType, payload: &Bytes) -> Address;
}

impl AddressPayloadExt for Address {
    fn payload(&self, env: &Env) -> Option<(AddressPayloadType, Bytes)> {
        let xdr = self.to_xdr(env);
        // Skip over ScVal discriminant because we know it is an ScAddress.
        let xdr = xdr.slice(4..);
        // Decode ScAddress
        let addr_type: BytesN<4> = xdr.slice(0..4).try_into().unwrap_optimized();
        match addr_type.to_array() {
            // Decode ScAddress::Account
            [0, 0, 0, 0] => {
                // Decode PublicKey
                let public_key_type: BytesN<4> = xdr.slice(4..8).try_into().unwrap_optimized();
                match public_key_type.to_array() {
                    // Decode PublicKey::PublicKeyTypeEd25519
                    [0, 0, 0, 0] => {
                        let ed25519 = xdr.slice(8..40);
                        Some((AddressPayloadType::AccountEd25519PublicKey, ed25519))
                    }
                    _ => None,
                }
            }
            // Decode ScAddress::Contract
            [0, 0, 0, 1] => {
                let hash = xdr.slice(4..36);
                Some((AddressPayloadType::ContractHash, hash))
            }
            _ => None,
        }
    }

    fn from_payload(env: &Env, payload_type: AddressPayloadType, payload: &Bytes) -> Address {
        // Build XDR header based on payload type:
        let header: &[u8] = match payload_type {
            AddressPayloadType::AccountEd25519PublicKey => &[
                0, 0, 0, 18, // ScVal::Address
                0, 0, 0, 0, // ScAddress::Account
                0, 0, 0, 0, // PublicKey::PublicKeyTypeEd25519
            ],
            AddressPayloadType::ContractHash => &[
                0, 0, 0, 18, // ScVal::Address
                0, 0, 0, 1, // ScAddress::Contract
            ],
        };

        let mut xdr = Bytes::from_slice(env, header);
        xdr.append(payload);

        Address::from_xdr(env, &xdr).unwrap_optimized()
    }
}
