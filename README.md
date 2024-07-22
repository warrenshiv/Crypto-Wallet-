# Alumni Association Management Platform

This project is a decentralized platform built on the Internet Computer, aiming to facilitate the management of alumni associations, events, mentorship requests, and communication within the alumni community. It leverages the power of the blockchain to ensure transparency and reliability in the management processes.

## Key Features

### Alumni Management
- **Create Alumni Profile**: Allows the creation of new alumni profiles with validation for input fields.
- **Get Alumni Profiles**: Retrieves all registered alumni profiles.
- **Get Alumni by ID**: Retrieves the profile of a specific alumni by their unique ID.
- **Search Alumni**: Searches for alumni by name or graduation year.

### Association Management
- **Create Association**: Allows the creation of new associations.
- **Get Associations**: Retrieves all registered associations.
- **Get Association by ID**: Retrieves the details of a specific association by its ID.
- **Join Association**: Allows an alumni to join an association.
- **Leave Association**: Allows an alumni to leave an association.

### Event Management
- **Create Event**: Allows the creation of new events within associations.
- **Get Events**: Retrieves all registered events.
- **Get Event by ID**: Retrieves the details of a specific event by its ID.
- **RSVP to Event**: Allows alumni to RSVP to events.

### Communication
- **Send Message to Association**: Allows alumni to send messages to association members.

### Mentorship Management
- **Request Mentorship**: Allows alumni to request mentorship from other alumni.
- **Approve Mentorship Request**: Approves a mentorship request.

### Error Handling
- **Not Found**: Returns an error if a requested resource (alumni, association, event) is not found.
- **Invalid Input**: Handles errors related to invalid input fields.



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