---
title: Rust API
---

The Powerledger blockchain, as it is an SVM based blockchain, is compatible with Solana's Rust 
crates that are [published to crates.io][crates.io] and can be found
[on docs.rs with the "solana-" prefix][docs.rs].

Powerledger utilises these Solana crates to interact with the Powerledger blockchain. Powerledger plans to switch to official Anza software releases once they are available.

[crates.io]: https://crates.io/search?q=solana-
[docs.rs]: https://docs.rs/releases/search?query=solana-

Some important crates:

- [`solana-program`] &mdash; Imported by programs running on SVM blockchains like the Powerledger blockchain, compiled
  to BPF. This crate contains many fundamental data types and is re-exported from
  [`solana-sdk`], which cannot be imported from a Solana program.

- [`solana-sdk`] &mdash; The basic off-chain SDK, it re-exports
  [`solana-program`] and adds more APIs on top of that. Most programs
  that do not run on-chain will import this.

- [`solana-client`] &mdash; For interacting with a blockchain node via the
  [JSON RPC API](jsonrpc-api).

- [`solana-cli-config`] &mdash; Loading and saving the Solana CLI tools configuration
  file.

- [`solana-clap-utils`] &mdash; Routines for setting up a CLI, using [`clap`],
  as used by the main Solana CLI that can be used to interact with the Powerledger blockchain. 
  Includes functions for loading all types of signers supported by the CLI.

[`solana-program`]: https://docs.rs/solana-program
[`solana-sdk`]: https://docs.rs/solana-sdk
[`solana-client`]: https://docs.rs/solana-client
[`solana-cli-config`]: https://docs.rs/solana-cli-config
[`solana-clap-utils`]: https://docs.rs/solana-clap-utils
[`clap`]: https://docs.rs/clap
