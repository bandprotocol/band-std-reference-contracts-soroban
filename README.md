# Soroban - Band Standard Reference Contract

This repository contains a Rust implementation of the Band `StandardReference` Soroban smart contract. The contract
provides functionality for querying prices (reference data) of supported symbols from the BandChain. It allows relayers
to update and retrieve reference data for the supported symbols

## Components

There are 2 key components in this repository:

1. [`src/contract.rs`](https://github.com/bandprotocol/band-std-reference-contracts-soroban/blob/main/src/contract.rs):
   contains core `StandardReference` contract functionality
2. [`examples/mock_consumer_contract/src/lib.rs`](https://github.com/bandprotocol/band-std-reference-contracts-soroban/blob/main/examples/mock_consumer_contract/src/lib.rs):
   contains an example implementation of a contract which uses the prices from the `StandardReference` contract.

## Standard Reference Contract

### Functionality

The `StandardReferenceTrait` trait defines the contract interface, which includes the following functions:

- `init`: Initializes the contract with the given admin address and adds the admin address to the relayers list.
- `upgrade`: Upgrades the contract to a new wasm code at the provided wasm hash.
- `version`: Returns the version of the contract.
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

The `StandardReference` struct implements the `StandardReferenceTrait` trait, providing the actual implementation for
the contract functions.

### Testing

The code includes a set of unit tests in the `tests` module. These tests cover various scenarios and verify the
correctness of the contract implementation. Here are some key test cases:

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

1. The admin has the authority to assign and revoke admin and relayer roles from any addresses.
2. The relayer is responsible for relaying the prices or data from BandChain to `StandardReference` contract via
   the `relay` and `force_relay` functions. The admin and relayer roles will be maintained by Band Protocol.
3. The consumer retrieves the prices for supported symbols via the `get_ref_data` and `get_reference_data` functions.
   Please refer to `MockConsumer` contract for guidance.

### RefData and ReferenceData

#### [RefData](https://github.com/bandprotocol/band-std-reference-contracts-soroban/blob/main/src/ref_data.rs)

The `RefData` struct is used to store reference data for a specific symbol pair. It includes information such as the
rate, resolve time, and request ID for a specific symbol.

```rust    
pub struct RefData {
    pub rate: u64,
    pub resolve_time: u64,
    pub request_id: u64,
}
```

The `RefData` struct has the following fields:

- `rate`: Represents the rate of the symbol against USD.
- `resolve_time`: Represents the Unix time when the reference data was resolved.
- `request_id`: Represents the ID of the request associated with the reference data.

#### [get_ref_data()](https://github.com/bandprotocol/band-std-reference-contracts-soroban/blob/2589e8211485af1ed2ac84f413bbe960df7af72d/src/contract.rs#L228)

This function is intended for obtaining reference data rate with Quote asset as USD.

```rust
fn get_ref_data(env: Env, symbols: Vec<Symbol>) ->
Result<Vec<RefData>, StandardReferenceError>
```

##### Parameters

`symbols`: A vector of symbols for which reference data is requested.

##### Return Value

The function returns a `Result` containing the reference data as a vector of `RefData` objects if successful. If there
is an error during the execution of the function, an `Err` variant of `StandardReferenceError` is returned.

#### [Reference Data](https://github.com/bandprotocol/band-std-reference-contracts-soroban/blob/main/src/reference_data.rs)

The `ReferenceData`struct is similar `RefData` but the quote asset can be specified. It includes information such as the
rate of the pair, and the last update times of the base and quote assets.

```rust    
pub struct ReferenceData {
    pub rate: u128,
    pub last_updated_base: u64,
    pub last_updated_quote: u64,
}
```

The `ReferenceData` struct has the following fields:

- `rate`: Represents the rate of the symbol pair. For example, the rate of BTC/USD.
- `last_updated_base`: Represents the Unix time when the base asset (e.g., BTC) was last updated.
- `last_updated_quote`: Represents the Unix time when the quote asset (e.g., USD) was last updated.

#### [get_reference_data()](https://github.com/bandprotocol/band-std-reference-contracts-soroban/blob/2589e8211485af1ed2ac84f413bbe960df7af72d/src/contract.rs#L252)

This function is used for obtaining the reference data rate by specifying both Base and Quote assets.

```rust
fn get_reference_data(
    env: Env,
    symbol_pairs: Vec<(Symbol, Symbol)>,
) -> Result<Vec<ReferenceData>, StandardReferenceError>
```

##### Parameters

`symbol_pairs`: A vector of symbol pairs (Base and Quote assets) for which reference data is requested.

##### Return Value

The function returns a `Result` containing the reference data as a vector of `ReferenceData` objects if successful. If
there is an error during the execution of the function, an `Err` variant of `StandardReferenceError` is returned.

## Mock Consumer Contract

The mock consumer contract provides an example of how to interact with a `StandardReference` contract. It provides
functionality to set the address of the standard reference contract and retrieve the price of symbol pairs.

The mock consumer contract provides the following functionality:

- `set_std_reference_address`: Sets the address of the `StandardReference` contract in the contract storage.
- `get_price_of`: Retrieves the price of the specified symbol pair from the `StandardReference` contract. It uses the
  stored address of the `StandardReference` contract to create a client and fetch the reference data.

## Example Usage

For example, if we want to query the price of BTC/USD, the demo contract below shows how this can be done.

```rust
#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Env, Symbol, Vec};

mod std_reference {
    soroban_sdk::contractimport!(file = "../../dist/std_reference.wasm");
}

pub struct Demo;

#[contractimpl]
impl Demo {
    pub fn set_std_reference_address(env: Env, std_reference_address: Address) {
        env.storage()
            .set(&DataKey::StdReferenceAddress, &std_reference_address);
    }

    pub fn demo(env: Env) -> u128 {
        let addr = env
            .storage()
            .get_unchecked(&DataKey::StdReferenceAddress)
            .unwrap();
        let client = std_reference::Client::new(&env, &addr);
        client
            .get_reference_data(&Vec::from_array(&env, [("BTC", "USD")]))
            .get_unchecked(0)
            .unwrap()
            .rate
    }
}
```

After the `StandardReference` contract address is set, the BTC/USD price can be queried via the `demo()` function. An
example of result from `demo()` would be: `30065900000000000000000`, which is equivalent to 30,065.90 USD. Note that the
output rate is multiplied by `1e18`.

## Setup - Build - Deploy

### Prerequisites

The followings are prerequisites required to compile the contract:

- A Rust toolchain
- An editor that supports Rust
- Soroban CLI

Please refer to [Soroban Documentation](https://soroban.stellar.org/docs/getting-started/setup) for installation
instructions.

### Build the Contract

To build a Soroban contract for deployment or local testing, use the following command:

```
make all
```

A `.wasm` file will be outputted in the target directory. The `.wasm` file is the built contract. For example:

```
target/wasm32-unknown-unknown/std_reference/std_reference.wasm
```

### Deploy the Contract

To deploy, use the deploy script at `scripts/deploy.sh`. For example:

```bash
scripts/deploy.sh -s <source> -n <network>
```

Where `<source>` is the source account and can either be a secret key, seed phrase or identity account and `<network>`
is the network to deploy to where the current network options are `futurenet` and `localnet`.
