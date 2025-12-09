# soroban-address-payload-ext

Extension trait for extracting and constructing Soroban `Address` from raw 32-byte payloads.

> [!WARNING]
> This library has not been audited for use in production contracts.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
soroban-address-payload-ext = "1"
```

### Extracting a payload from an Address

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

### Constructing an Address from a payload

```rust
use soroban_sdk::{Address, Bytes, Env, bytes};
use soroban_address_payload_ext::{AddressPayloadExt, AddressPayloadType};

let env = Env::default();
let hash: Bytes = bytes!(
    &env,
    0xd7928b72c2703ccfeaf7eb9ff4ef4d504a55a8b979fc9b450ea2c842b4d1ce61
);
let address = Address::from_payload(&env, AddressPayloadType::ContractHash, &hash);
```
