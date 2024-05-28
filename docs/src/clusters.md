---
title: Powerledger Blockchain Clusters
---

Powerledger maintains two different public clusters with different purposes.

Before you begin make sure you have first
[installed the command line tools](cli/install-solana-cli-tools.md)

Explorer:

- [http://explorer.powerledger.io/](https://explorer.powerledger.io/).

## Testnet

- Testnet serves as a playground for anyone who wants to take the Powerledger blockchain for a
  test drive, as a user, token holder, app developer, or validator.
- Application developers should target Testnet.
- Potential validators should target Testnet.
- Key differences between Testnet and Mainnet:
  - Testnet tokens are **not real**
  - Testnet includes a token faucet for airdrops for application testing
  - Testnet may be subject to ledger resets
  - Testnet typically runs the same software release branch version as Mainnet,
    but may run a newer minor release version than Mainnet.
- Gossip entrypoint for Testnet: `161.97.170.10:8001`

- RPC URL for Devnet: `https://powr-api.testnet.powerledger.io`

##### Example `solana` command-line configuration

Powerledger blockchain uses the `solana` binary to run and interact via command line, as it is an SVM based blockchain.

```bash
solana config set --url https://powr-api.testnet.powerledger.io
```

##### Example `solana-validator` command-line

```bash
$ solana-validator \
    --identity ~/validator-keypair.json \
    --vote-account ~/vote-account-keypair.json \
    --known-validator EeMpsFwCcU6MAjqVKFHrYgiju3Z55nJTnzPqwQebXZKW \
    --known-validator B8TZaKjjQP8RvcoxKKtDG12VuFpsNXbqahvfZtjtjX4q \
    --known-validator BzE83SMXuqKwAPuFR6Yi5hQPD96Myq3oM7Ksg5agZXxu \
    --known-validator FuctYcng8Sv8jTd8TzdBg4F7vr7dsGDy24hm77zSsfYT \
    --only-known-rpc \
    --ledger ledger \
    --rpc-port 8899 \
    --dynamic-port-range 8000-8020 \
    --entrypoint 161.97.170.10:8001 \
    --wal-recovery-mode skip_any_corrupted_record \
    --no-genesis-fetch \
    --limit-ledger-size
```

The [`--known-validator`s](running-validator/validator-start.md#known-validators)
are operated by Powerledger

## Mainnet

A permissionless, persistent cluster for Powerledger users, builders, validators and token holders.

- Tokens that are issued on Mainnet are **real** POWR
- Gossip entrypoint for Mainnet: `powr-entrypoint-1.mainnet.powerledger.io:8001`


- RPC URL for Mainnet Beta: `https://powr-api.mainnet.powerledger.io`

##### Example `solana` command-line configuration

```bash
solana config set --url https://powr-api.mainnet.powerledger.io
```

##### Example `solana-validator` command-line

```bash
$ solana-validator \
    --identity ~/validator-keypair.json \
    --vote-account ~/vote-account-keypair.json \
    --known-validator 6H69egwLDXs27r7tfUu5oxesbcYbi6YL8JW2D22jkBEi \
    --known-validator 6YDqUUE3JBA9sVLNNiWYQR7moAPyNfDsBKPEvGcuEfQ2 \
    --known-validator CcS24i73biPiNMiUmdxQjj8mQFXayK79yr2AxsmvPaRo \
    --known-validator 8oATwTdKJqUvWMkUjFDBVW23HDF5DbVRskxWCypXwRRB \
    --only-known-rpc \
    --ledger ledger \
    --rpc-port 8899 \
    --private-rpc \
    --dynamic-port-range 8000-8020 \
    --entrypoint powr-entrypoint-1.mainnet.powerledger.io:8001 \
    --entrypoint powr-entrypoint-2.mainnet.powerledger.io:8001 \
    --expected-genesis-hash B1xJgyn5UyWw2itxwwctHL5fLxY9ZbnsDf7AG2WXbpGY \
    --wal-recovery-mode skip_any_corrupted_record \
    --limit-ledger-size
```

All four [`--known-validator`s](running-validator/validator-start.md#known-validators)
are operated by Powerledger
