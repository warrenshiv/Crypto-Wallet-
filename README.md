
# Crypto Wallet

This project implements a decentralized user and transaction management system on the Internet Computer. It uses Rust and various IC (Internet Computer) libraries for handling stable structures, candid, and CDK.

## Features

- User Creation with validation
- Fund Deposit to user accounts
- Sending transactions between users
- Redeeming points
- Retrieving transaction history
- Checking user balance and points

## Usage

### Create a User

To create a user, call the `create_user` method with a `UserPayload`:

```rust
dfx canister call your_canister create_user '(record {first_name="John"; last_name="Doe"; email="john.doe@example.com"; phone_number="+1234567890"})'
```

### Deposit Funds

To deposit funds to a user's account, call the `deposit_funds` method with a `DepositPayload`:

```rust
dfx canister call your_canister deposit_funds '(record {user_id=1; amount=1000})'
```

### Send Transaction

To send a transaction, call the `send_transaction` method with a `TransactionPayload`:

```rust
dfx canister call your_canister send_transaction '(record {from_user_id=1; to_user_id=2; amount=500})'
```

### Redeem Points

To redeem points, call the `redeem_points` method with a `PointsPayload`:

```rust
dfx canister call your_canister redeem_points '(record {user_id=1; points=50})'
```

### Get Transaction History

To get the transaction history for a user, call the `get_transaction_history` method:


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```