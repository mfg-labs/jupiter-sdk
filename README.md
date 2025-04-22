# Jupiter SDK

This Rust SDK was generated from an [IDL](https://book.anchor-lang.com/anchor/idl.html) file and provides tools for interacting with the `Jupiter` Solana program.

## 📦 Installation

To use this crate, add the following to your `Cargo.toml`:

```toml
jupiter_sdk = { git = "https://github.com/mfg-labs/jupiter-sdk", branch = "main" }
```

Or import it from a local path if you're testing locally:

```toml
jupiter_sdk = { path = "../jupiter-sdk" }
```

## 🧩 Features

- **Instructions**: Structs and serialization logic for all program instructions.
- **Accounts**: Anchor-compatible account representations.
- **CPI**: Context builders and invocation helpers for cross-program invocation.
- **RPC**: Structs for client-side account metas and instruction construction.
- **I11n**: Introspection helpers to decode raw transactions.
- **Events**: Strongly-typed deserialization of emitted events.

## 🚀 Usage

```rust
use anchor_lang::prelude::*;
use jupiter_sdk::instructions::*;
use jupiter_sdk::rpc::*;

// Construct the arguments for the instruction
let ix = MyInstructionName {
    field_one: 42,
    field_two: true,
};

// Serialize the instruction
let mut data = Vec::new();
data.extend_from_slice(&MyInstructionName::DISCRIMINATOR);
ix.serialize(&mut data).unwrap();

// Build the Solana instruction
let accounts = MyInstructionNameRpc {
    authority: Pubkey::new_unique(),
    some_account: Pubkey::new_unique(),
};
let metas = accounts.to_account_metas(None);

let instruction = solana_program::instruction::Instruction {
    program_id: ID, // imported from the SDK
    accounts: metas,
    data,
};
```
