# pwman

Step 1: Basic CLI password manager in Rust.

- Add accounts: `cargo run -- add <service> <password>`
- List accounts: `cargo run -- list`
- Get password: `cargo run -- get <service>`

Stores raw passwords in `vault.json` for now.
