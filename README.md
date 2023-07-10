# Soroban - Band Standard Reference Contract

This repository contains a Rust implementation of the Band `StandardReference` Soroban smart contract. The contract provides functionality for querying prices (reference data) of supported symbols from the BandChain. It allows relayers to update and retrieve reference data for the supported symbols

## Components
There are 2 key components in this repository:
1. `src/contract.rs`: contains core `StandardReference` contract functionality
2. `examples/mock_consumer_contract/src/lib.rs`: contains an example implementation of a contract which uses the prices from the `StandardReference` contract.

## Standard Reference Contract

### Functionality

The `StandardReferenceTrait` trait defines the contract interface, which includes the following functions:

- `init`: Initializes the contract with the given admin address and adds the admin address to the relayers list.
- `upgrade`: Upgrades the contract to a new wasm code at the provided wasm hash.
- `version`: Returns the version of the contract.
- `address`: Returns the address of the contract.
- `current_admin`: Returns the current admin address.
- `transfer_admin`: Transfers the admin role to the new admin address and revokes relayer status from the old admin.
- `is_relayer`: Checks if the given address is a relayer.
- `add_relayers`: Adds the given addresses to the relayers list.
- `remove_relayers`: Removes the given addresses from the relayers list.
- `relay`: Relays symbol rates to the contract. The caller must be a relayer.
- `force_relay`: Forces relay of symbol rates to the contract. The caller must be a relayer.
- `delist`: Delists symbols from the contract. The caller must be a relayer.
- `get_ref_data`: Retrieves reference data for the specified symbols.
- `get_reference_data`: Retrieves reference data for the specified symbol pairs.

The `StandardReference` struct implements the `StandardReferenceTrait` trait, providing the actual implementation for the contract functions.

### Testing

The code includes a set of unit tests in the `tests` module. These tests cover various scenarios and verify the correctness of the contract implementation. Here are some key test cases:

- `test_reinit`: Verifies that initializing the contract multiple times results in a panic.
- `test_transfer_admin`: Tests the transfer of admin role to a new admin address.
- `test_add_relayers`: Tests adding relayers to the contract.
- `test_remove_relayers`: Tests removing relayers from the contract.
- `test_relay`: Tests the relay of symbol rates by relayers and verifies the updated reference data.
- `test_unauthorized_relay`: Ensures that unauthorized relayers cannot update the reference data.
- `test_force_relay`: Tests the forced relay of symbol rates by relayers and verifies the updated reference data.
- `test_delist`: Tests the delisting of symbols from the contract and verifies the removal of reference data.

### Usage

There are three types of players: admin, relayer and consumer. 
- The admin has the authority to assign and revoke admin and relayer roles from any addresses. 
- The relayer is responsible for relaying the prices or data from BandChain to `StandardReference` contract via the `relay` and `force_relay` functions. The admin and relayer roles will be maintained by Band Protocol.
- The consumer retrieves the prices for supported symbols via the `get_ref_data` and `get_reference_data` functions. Please refer to `MockConsumer` contract for guidance. 

### RefData and ReferenceData

#### RefData

The `RefData` struct is used to store reference data for a specific symbol pair. It includes information such as the rate, resolve time, and request ID for a specific symbol. This function is intended for obtaining reference data rate with quote asset as USD. See `src/ref_data.rs`.

The `RefData` struct has the following fields:

- `rate`: Represents the rate of the symbol against USD.
- `resolve_time`: Represents the Unix time when the reference data was resolved.
- `request_id`: Represents the ID of the request associated with the reference data.

#### Reference Data

The `ReferenceData`struct is similar `RefData` but the quote asset can be specified. It includes information such as the rate of the pair, and the last update times of the base and quote assets. See `src/reference_data.rs`.

The `ReferenceData` struct has the following fields:

- `rate`: Represents the rate of the symbol pair. For example, the rate of BTC/USD.
- `last_updated_base`: Represents the Unix time when the base asset (e.g., BTC) was last updated.
- `last_updated_quote`: Represents the Unix time when the quote asset (e.g., USD) was last updated.

## Mock Consumer Contract

The mock consumer contract provides an example of how to interact with a `StandardReference` contract. It provides functionality to set the address of the standard reference contract and retrieve the price of symbol pairs.

The mock consumer contract provides the following functionality:

- `set_std_reference_address`: Sets the address of the `StandardReference` contract in the contract storage.
- `get_price_of`: Retrieves the price of the specified symbol pair from the `StandardReference` contract. It uses the stored address of the `StandardReference` contract to create a client and fetch the reference data.
