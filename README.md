# soroban-address-payload-ext

Extension trait for extracting the 32-byte payload from a Soroban `Address`.

## Usage

```rust
use soroban_sdk::{Address, Env};
use soroban_address_payload_ext::{AddressPayloadExt, AddressPayloadType};

let env = Env::default();
let address = Address::from_str(&env, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");

if let Some((payload_type, payload)) = address.payload(&env) {
    match payload_type {
        AddressPayloadType::ContractHash => {
            // 32-byte contract hash
        }
        AddressPayloadType::AccountEd25519PublicKey => {
            // 32-byte ed25519 public key
        }
    }
}
```

## Types

- `AddressPayloadType::ContractHash` - Contract address (C...)
- `AddressPayloadType::AccountEd25519PublicKey` - Account address (G...)
