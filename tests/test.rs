use soroban_address_payload_ext::{AddressPayloadExt, AddressPayloadType};
use soroban_sdk::{bytes, testutils::EnvTestConfig, Address, Bytes, Env, String};

#[test]
fn test_payload() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });

    // Test cases: (address, expected_type, expected_payload)
    let test_cases: [(&str, AddressPayloadType, Bytes); 2] = [
        // Contract address (C...)
        (
            "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
            AddressPayloadType::ContractHash,
            bytes!(
                &env,
                0xd7928b72c2703ccfeaf7eb9ff4ef4d504a55a8b979fc9b450ea2c842b4d1ce61
            ),
        ),
        // Account address (G...)
        (
            "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGSNFHEYVXM3XOJMDS674JZ",
            AddressPayloadType::AccountEd25519PublicKey,
            bytes!(
                &env,
                0x899b2840ed5636c56ddc5f14b23975f79f1ba2388d2694e4c56ecdddc960e5ef
            ),
        ),
    ];

    for (address, payload_type, payload) in test_cases {
        let address = String::from_str(&env, address);
        let address = Address::from_string(&address);

        // Test payload:
        {
            let (actual_payload_type, actual_payload) = address.payload(&env).unwrap();
            assert_eq!(actual_payload_type, payload_type);
            assert_eq!(actual_payload, payload);
        }

        // Test from_payload:
        {
            let actual_address = Address::from_payload(&env, payload_type, &payload);
            assert_eq!(actual_address, address);
        }
    }
}
