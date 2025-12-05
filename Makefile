.phony: sdk-versions test all

# output soroban-sdk versions as json array (for ci)
sdk-versions:
	@curl -sA 'soroban-address-payload-ext' 'https://crates.io/api/v1/crates/soroban-sdk/versions' | \
		jq -c '[.versions[].num | select(test("^[0-9]+\\.[0-9]+\\.[0-9]+$$")) | select((split(".")[0] | tonumber) >= 20)] | group_by(split(".")[0]) | map(max_by(split(".") | map(tonumber)))'

# Test with a specific soroban-sdk version: make test SDK=23.2.1
test:
ifdef SDK
	cargo update soroban-sdk --precise $(SDK)
endif
	cargo build
	cargo test

# Build and test against all supported soroban-sdk versions
test-all:
	@set -e; \
	for version in $$($(MAKE) -s sdk-versions | jq -r '.[]'); do \
		$(MAKE) test SDK=$$version; \
	done
