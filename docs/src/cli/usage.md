---
title: CLI Usage Reference
---

Powerledger uses the [solana-cli crate](https://crates.io/crates/solana-cli) to provide a command-line interface tool for the Powerledger blockchain, as it is based on the Solana codebase, and it compatible with the CLI tooling. Soon, the Anza binaries will be used instead of the Solana binaries.

This means that to interact with the Powerledger blockchain via the CLI, you must use the `solana` command, and it will show SOL instead of POWR on the command line when checking balances etc. for the time being.

To use the Solana CLI with the Powerledger blockchain, you must first set your configuration to use the Powerledger blockchain cluster. This is done by setting and RPC URL like below:

### Configure CLI to use Powerledger blockchain
```bash
// Configure to use mainnet
$ solana config set --url https://powr-api.mainnet.powerledger.io

// Configure to use testnet
$ solana config set --url https://powr-api.testnet.powerledger.io
```

After this, any commands used in the CLI will be pointed towards the appropriate Powerledger blockchain cluster.

### Getting extra command help
You can type `--help` after any command from the CLI tools to get more information about how it is used. For example:
```text
// Get help about solana balance command
$ solana balance --help

// Output
solana-balance
Get your balance

USAGE:
    solana balance [FLAGS] [OPTIONS] [ACCOUNT_ADDRESS]

FLAGS:
    -h, --help                           Prints help information
        --lamports                       Display balance in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
        --use-quic                       Use QUIC when sending transactions.
        --use-udp                        Use UDP when sending transactions.
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           /Users/michael/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for Solana's JSON RPC or moniker (or their first letter): [mainnet-beta,
                                           testnet, devnet, localhost]
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the solana cluster

ARGS:
    <ACCOUNT_ADDRESS>    Account balance to check. Address is one of:
                           * a base58-encoded public key
                           * a path to a keypair file
                           * a hyphen; signals a JSON-encoded keypair on stdin
                           * the 'ASK' keyword; to recover a keypair via its seed phrase
                           * a hardware wallet keypair URL (i.e. usb://ledger)

```

## Examples

### Get Pubkey

```bash
// Command
$ solana-keygen pubkey

// Return
<PUBKEY>
```

### Airdrop POWR/Sparks

```bash
// Command
$ solana airdrop 1

// Return
"1 SOL"
```

### Get Balance

```bash
// Command
$ solana balance

// Return
"3.00050001 SOL"
```

### Confirm Transaction

```bash
// Command
$ solana confirm <TX_SIGNATURE>

// Return
"Confirmed" / "Not found" / "Transaction failed with error <ERR>"
```

### Deploy program

```bash
// Command
$ solana program deploy <PATH>

// Return
<PROGRAM_ID>
```

## Usage
### solana-cli
```text
solana-cli 1.13.7 (src:devbuild; feat:2933019217)
Blockchain, Rebuilt for Scale

USAGE:
    solana [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

SUBCOMMANDS:
    account                              Show the contents of an account
    address                              Get your public key
    airdrop                              Request POWR from a faucet
    authorize-nonce-account              Assign account authority to a new entity
    balance                              Get your balance
    block                                Get a confirmed block
    block-height                         Get current block height
    block-production                     Show information about block production
    block-time                           Get estimated production time of a block
    catchup                              Wait for a validator to catch up to the cluster
    close-vote-account                   Close a vote account and withdraw all funds remaining
    cluster-date                         Get current cluster date, computed from genesis creation time and network
                                         time
    cluster-version                      Get the version of the cluster entrypoint
    completion                           Generate completion scripts for various shells
    config                               Solana command-line tool configuration settings
    confirm                              Confirm transaction by signature
    create-address-with-seed             Generate a derived account address with a seed
    create-nonce-account                 Create a nonce account
    create-stake-account                 Create a stake account
    create-stake-account-checked         Create a stake account, checking the withdraw authority as a signer
    create-vote-account                  Create a vote account
    deactivate-stake                     Deactivate the delegated stake from the stake account
    decode-transaction                   Decode a serialized transaction
    delegate-stake                       Delegate stake to a vote account
    epoch                                Get current epoch
    epoch-info                           Get information about the current epoch
    feature                              Runtime feature management
    fees                                 Display current cluster fees (Deprecated in v1.8.0)
    first-available-block                Get the first available block in the storage
    genesis-hash                         Get the genesis hash
    gossip                               Show the current gossip network nodes
    help                                 Prints this message or the help of the given subcommand(s)
    inflation                            Show inflation information
    largest-accounts                     Get addresses of largest cluster accounts
    leader-schedule                      Display leader schedule
    live-slots                           Show information about the current slot progression
    logs                                 Stream transaction logs
    merge-stake                          Merges one stake account into another
    new-nonce                            Generate a new nonce, rendering the existing nonce useless
    nonce                                Get the current nonce value
    nonce-account                        Show the contents of a nonce account
    ping                                 Submit transactions sequentially
    program                              Program management
    rent                                 Calculate per-epoch and rent-exempt-minimum values for a given account data
                                         field length.
    resolve-signer                       Checks that a signer is valid, and returns its specific path; useful for
                                         signers that may be specified generally, eg. usb://ledger
    slot                                 Get current slot
    split-stake                          Duplicate a stake account, splitting the tokens between the two
    stake-account                        Show the contents of a stake account
    stake-authorize                      Authorize a new signing keypair for the given stake account
    stake-authorize-checked              Authorize a new signing keypair for the given stake account, checking the
                                         authority as a signer
    stake-history                        Show the stake history
    stake-set-lockup                     Set Lockup for the stake account
    stake-set-lockup-checked             Set Lockup for the stake account, checking the new authority as a signer
    stakes                               Show stake account information
    supply                               Get information about the cluster supply of POWR
    transaction-count                    Get current transaction count
    transaction-history                  Show historical transactions affecting the given address from newest to
                                         oldest
    transfer                             Transfer funds between system accounts
    upgrade-nonce-account                One-time idempotent upgrade of legacy nonce versions in order to bump them
                                         out of chain blockhash domain.
    validator-info                       Publish/get Validator info on Solana
    validators                           Show summary information about the current validators
    vote-account                         Show the contents of a vote account
    vote-authorize-voter                 Authorize a new vote signing keypair for the given vote account
    vote-authorize-voter-checked         Authorize a new vote signing keypair for the given vote account, checking
                                         the new authority as a signer
    vote-authorize-withdrawer            Authorize a new withdraw signing keypair for the given vote account
    vote-authorize-withdrawer-checked    Authorize a new withdraw signing keypair for the given vote account,
                                         checking the new authority as a signer
    vote-update-commission               Update the vote account's commission
    vote-update-validator                Update the vote account's validator identity
    wait-for-max-stake                   Wait for the max stake of any one node to drop below a percentage of total.
    withdraw-from-nonce-account          Withdraw POWR from the nonce account
    withdraw-from-vote-account           Withdraw from a vote account into a specified account
    withdraw-stake                       Withdraw the unstaked POWR from the stake account
```

#### solana-account
```text
solana-account 
Show the contents of an account

USAGE:
    solana account [FLAGS] [OPTIONS] <ACCOUNT_ADDRESS>

FLAGS:
    -h, --help                           Prints help information
        --lamports                       Display balance in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
    -o, --output-file <FILEPATH>           Write the account data to this file
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <ACCOUNT_ADDRESS>    Account key URI. , one of:
                           * a base58-encoded public key
                           * a path to a keypair file
                           * a hyphen; signals a JSON-encoded keypair on stdin
                           * the 'ASK' keyword; to recover a keypair via its seed phrase
                           * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-address
```text
solana-address 
Get your public key

USAGE:
    solana address [FLAGS] [OPTIONS]

FLAGS:
        --confirm-key                    Confirm key on device; only relevant if using remote wallet
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-airdrop
```text
solana-airdrop 
Request POWR from a faucet

USAGE:
    solana airdrop [FLAGS] [OPTIONS] <AMOUNT> [RECIPIENT_ADDRESS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <AMOUNT>               The airdrop amount to request, in POWR
    <RECIPIENT_ADDRESS>    The account address of airdrop recipient. , one of:
                             * a base58-encoded public key
                             * a path to a keypair file
                             * a hyphen; signals a JSON-encoded keypair on stdin
                             * the 'ASK' keyword; to recover a keypair via its seed phrase
                             * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-authorize-nonce-account
```text
solana-authorize-nonce-account 
Assign account authority to a new entity

USAGE:
    solana authorize-nonce-account [FLAGS] [OPTIONS] <NONCE_ACCOUNT_ADDRESS> <AUTHORITY_PUBKEY>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <NONCE_ACCOUNT_ADDRESS>    Address of the nonce account. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
    <AUTHORITY_PUBKEY>         Account to be granted authority of the nonce account. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-balance
```text
solana-balance 
Get your balance

USAGE:
    solana balance [FLAGS] [OPTIONS] [ACCOUNT_ADDRESS]

FLAGS:
    -h, --help                           Prints help information
        --lamports                       Display balance in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <ACCOUNT_ADDRESS>    The account address of the balance to check. , one of:
                           * a base58-encoded public key
                           * a path to a keypair file
                           * a hyphen; signals a JSON-encoded keypair on stdin
                           * the 'ASK' keyword; to recover a keypair via its seed phrase
                           * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-block
```text
solana-block 
Get a confirmed block

USAGE:
    solana block [FLAGS] [OPTIONS] [SLOT]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <SLOT>    
```

#### solana-block-height
```text
solana-block-height 
Get current block height

USAGE:
    solana block-height [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-block-production
```text
solana-block-production 
Show information about block production

USAGE:
    solana block-production [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --epoch <epoch>                    Epoch to show block production for [default: current epoch]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --slot-limit <slot_limit>          Limit results to this many slots from the end of the epoch [default: full
                                           epoch]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-block-time
```text
solana-block-time 
Get estimated production time of a block

USAGE:
    solana block-time [FLAGS] [OPTIONS] [SLOT]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <SLOT>    Slot number of the block to query
```

#### solana-catchup
```text
solana-catchup 
Wait for a validator to catch up to the cluster

USAGE:
    solana catchup [FLAGS] [OPTIONS] [ARGS]

FLAGS:
        --follow                         Continue reporting progress even after the validator has caught up
    -h, --help                           Prints help information
        --log                            Don't update the progress inplace; instead show updates with its own new lines
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --our-localhost <PORT>             Guess Identity pubkey and validator rpc node assuming local (possibly
                                           private) validator [default: 8899]
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <OUR_VALIDATOR_PUBKEY>    Identity pubkey of the validator, one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
    <OUR_URL>                 JSON RPC URL for validator, which is useful for validators with a private RPC service
```

#### solana-close-vote-account
```text
solana-close-vote-account 
Close a vote account and withdraw all funds remaining

USAGE:
    solana close-vote-account [FLAGS] [OPTIONS] <VOTE_ACCOUNT_ADDRESS> <RECIPIENT_ADDRESS>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --authorized-withdrawer <AUTHORIZED_KEYPAIR>    Authorized withdrawer [default: cli config keypair]
        --commitment <COMMITMENT_LEVEL>
            Return information at the selected commitment level [possible values: processed, confirmed, finalized]

    -C, --config <FILEPATH>
            Configuration file to use [default: ~/.config/solana/cli/config.yml]

        --fee-payer <KEYPAIR>
            Specify the fee-payer account. This may be a keypair file, the ASK keyword 
            or the pubkey of an offline signer, provided an appropriate --signer argument 
            is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>
             URL for cluster's JSON RPC.

    -k, --keypair <KEYPAIR>                             Filepath or URL to a keypair
        --with-memo <MEMO>                              Specify a memo string to include in the transaction.
        --output <FORMAT>
            Return information in specified output format [possible values: json, json-compact]

        --ws <URL>                                      WebSocket URL for the cluster

ARGS:
    <VOTE_ACCOUNT_ADDRESS>    Vote account to be closed. , one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
    <RECIPIENT_ADDRESS>       The recipient of all withdrawn POWR. , one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-cluster-date
```text
solana-cluster-date 
Get current cluster date, computed from genesis creation time and network time

USAGE:
    solana cluster-date [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-cluster-version
```text
solana-cluster-version 
Get the version of the cluster entrypoint

USAGE:
    solana cluster-version [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-completion
```text
solana-completion 
Generate completion scripts for various shells

USAGE:
    solana completion [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
    -s, --shell <shell>                     [default: bash]  [possible values: bash, fish, zsh, powershell, elvish]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-config
```text
solana-config 
Solana command-line tool configuration settings

USAGE:
    solana config [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

SUBCOMMANDS:
    export-address-labels    Export the current address labels
    get                      Get current config settings
    help                     Prints this message or the help of the given subcommand(s)
    import-address-labels    Import a list of address labels
    set                      Set a config setting
```

#### solana-confirm
```text
solana-confirm 
Confirm transaction by signature

USAGE:
    solana confirm [FLAGS] [OPTIONS] <TRANSACTION_SIGNATURE>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <TRANSACTION_SIGNATURE>    The transaction signature to confirm

Note: This will show more detailed information for finalized transactions with verbose mode (-v/--verbose).

Account modes:
  |srwx|
    s: signed
    r: readable (always true)
    w: writable
    x: program account (inner instructions excluded)
```

#### solana-create-address-with-seed
```text
solana-create-address-with-seed 
Generate a derived account address with a seed

USAGE:
    solana create-address-with-seed [FLAGS] [OPTIONS] <SEED_STRING> <PROGRAM_ID>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --from <FROM_PUBKEY>               From (base) key, [default: cli config keypair]. , one of:
                                             * a base58-encoded public key
                                             * a path to a keypair file
                                             * a hyphen; signals a JSON-encoded keypair on stdin
                                             * the 'ASK' keyword; to recover a keypair via its seed phrase
                                             * a hardware wallet keypair URL (i.e. usb://ledger)
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <SEED_STRING>    The seed.  Must not take more than 32 bytes to encode as utf-8
    <PROGRAM_ID>     The program_id that the address will ultimately be used for, 
                     or one of NONCE, STAKE, and VOTE keywords
```

#### solana-create-nonce-account
```text
solana-create-nonce-account 
Create a nonce account

USAGE:
    solana create-nonce-account [FLAGS] [OPTIONS] <ACCOUNT_KEYPAIR> <AMOUNT>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce-authority <PUBKEY>         Assign noncing authority to another entity. , one of:
                                             * a base58-encoded public key
                                             * a path to a keypair file
                                             * a hyphen; signals a JSON-encoded keypair on stdin
                                             * the 'ASK' keyword; to recover a keypair via its seed phrase
                                             * a hardware wallet keypair URL (i.e. usb://ledger)
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --seed <STRING>                    Seed for address generation; if specified, the resulting account will be at a
                                           derived address of the NONCE_ACCOUNT pubkey
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <ACCOUNT_KEYPAIR>    Keypair of the nonce account to fund
    <AMOUNT>             The amount to load the nonce account with, in POWR; accepts keyword ALL
```

#### solana-create-stake-account
```text
solana-create-stake-account 
Create a stake account

USAGE:
    solana create-stake-account [FLAGS] [OPTIONS] <STAKE_ACCOUNT_KEYPAIR> <AMOUNT>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>             Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>     Return information at the selected commitment level [possible values:
                                            processed, confirmed, finalized]
    -C, --config <FILEPATH>                 Configuration file to use [default:
                                            ~/.config/solana/cli/config.yml]
        --custodian <PUBKEY>                Authority to modify lockups. , one of:
                                              * a base58-encoded public key
                                              * a path to a keypair file
                                              * a hyphen; signals a JSON-encoded keypair on stdin
                                              * the 'ASK' keyword; to recover a keypair via its seed phrase
                                              * a hardware wallet keypair URL (i.e. usb://ledger)
        --fee-payer <KEYPAIR>               Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                            or the pubkey of an offline signer, provided an appropriate --signer
                                            argument 
                                            is also passed. Defaults to the client keypair.
        --from <KEYPAIR>                    Source account of funds [default: cli config keypair]
    -u, --url <URL_OR_MONIKER>              URL for cluster's JSON RPC
    -k, --keypair <KEYPAIR>                 Filepath or URL to a keypair
        --lockup-date <RFC3339 DATETIME>    The date and time at which this account will be available for withdrawal
        --lockup-epoch <NUMBER>             The epoch height at which this account will be available for withdrawal
        --with-memo <MEMO>                  Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                    Provide the nonce account to use when creating a nonced 
                                            transaction. Nonced transactions are useful when a transaction 
                                            requires a lengthy signing process.
        --nonce-authority <KEYPAIR>         Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                   Return information in specified output format [possible values: json, json-
                                            compact]
        --seed <STRING>                     Seed for address generation; if specified, the resulting account will be at
                                            a derived address of the STAKE_ACCOUNT_KEYPAIR pubkey
        --signer <PUBKEY=SIGNATURE>...      Provide a public-key/signature pair for the transaction
        --stake-authority <PUBKEY>          Authorized staker [default: cli config keypair]
        --ws <URL>                          WebSocket URL for the cluster
        --withdraw-authority <PUBKEY>       Authorized withdrawer [default: cli config keypair]

ARGS:
    <STAKE_ACCOUNT_KEYPAIR>    Stake account to create (or base of derived address if --seed is used)
    <AMOUNT>                   The amount to send to the stake account, in POWR; accepts keyword ALL
```

#### solana-create-stake-account-checked
```text
solana-create-stake-account-checked 
Create a stake account, checking the withdraw authority as a signer

USAGE:
    solana create-stake-account-checked [FLAGS] [OPTIONS] <STAKE_ACCOUNT_KEYPAIR> <AMOUNT>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
        --from <KEYPAIR>                   Source account of funds [default: cli config keypair]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process.
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --seed <STRING>                    Seed for address generation; if specified, the resulting account will be at a
                                           derived address of the STAKE_ACCOUNT_KEYPAIR pubkey
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --stake-authority <PUBKEY>         Authorized staker [default: cli config keypair]
        --ws <URL>                         WebSocket URL for the cluster
        --withdraw-authority <KEYPAIR>     Authorized withdrawer [default: cli config keypair]

ARGS:
    <STAKE_ACCOUNT_KEYPAIR>    Stake account to create (or base of derived address if --seed is used)
    <AMOUNT>                   The amount to send to the stake account, in POWR; accepts keyword ALL
```

#### solana-create-vote-account
```text
solana-create-vote-account 
Create a vote account

USAGE:
    solana create-vote-account [FLAGS] [OPTIONS] <ACCOUNT_KEYPAIR> <IDENTITY_KEYPAIR> <WITHDRAWER_PUBKEY>

FLAGS:
        --allow-unsafe-authorized-withdrawer    Allow an authorized withdrawer pubkey to be identical to the validator
                                                identity account pubkey or vote account pubkey, which is normally an
                                                unsafe configuration and should be avoided.
        --dump-transaction-message              Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                                  Prints help information
        --no-address-labels                     Do not use address labels in the output
        --sign-only                             Sign the transaction offline
        --skip-seed-phrase-validation           Skip validation of seed phrases. Use this if your phrase does not use
                                                the BIP39 official English word list
    -V, --version                               Prints version information
    -v, --verbose                               Show additional information

OPTIONS:
        --authorized-voter <VOTER_PUBKEY>    Public key of the authorized voter [default: validator identity pubkey]. ,
                                             one of:
                                               * a base58-encoded public key
                                               * a path to a keypair file
                                               * a hyphen; signals a JSON-encoded keypair on stdin
                                               * the 'ASK' keyword; to recover a keypair via its seed phrase
                                               * a hardware wallet keypair URL (i.e. usb://ledger)
        --blockhash <BLOCKHASH>              Use the supplied blockhash
        --commission <PERCENTAGE>            The commission taken on reward redemption (0-100) [default: 100]
        --commitment <COMMITMENT_LEVEL>      Return information at the selected commitment level [possible values:
                                             processed, confirmed, finalized]
    -C, --config <FILEPATH>                  Configuration file to use [default:
                                             ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>                Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                             or the pubkey of an offline signer, provided an appropriate --signer
                                             argument 
                                             is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>               URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                  Filepath or URL to a keypair
        --with-memo <MEMO>                   Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                     Provide the nonce account to use when creating a nonced 
                                             transaction. Nonced transactions are useful when a transaction 
                                             requires a lengthy signing process.
        --nonce-authority <KEYPAIR>          Provide the nonce authority keypair to use when signing a nonced
                                             transaction
        --output <FORMAT>                    Return information in specified output format [possible values: json, json-
                                             compact]
        --seed <STRING>                      Seed for address generation; if specified, the resulting account will be at
                                             a derived address of the VOTE ACCOUNT pubkey
        --signer <PUBKEY=SIGNATURE>...       Provide a public-key/signature pair for the transaction
        --ws <URL>                           WebSocket URL for the cluster

ARGS:
    <ACCOUNT_KEYPAIR>      Vote account keypair to create
    <IDENTITY_KEYPAIR>     Keypair of validator that will vote with this account
    <WITHDRAWER_PUBKEY>    Public key of the authorized withdrawer, one of:
                             * a base58-encoded public key
                             * a path to a keypair file
                             * a hyphen; signals a JSON-encoded keypair on stdin
                             * the 'ASK' keyword; to recover a keypair via its seed phrase
                             * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-deactivate-stake
```text
solana-deactivate-stake 
Deactivate the delegated stake from the stake account

USAGE:
    solana deactivate-stake [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --seed <STRING>                    Seed for address generation; if specified, the resulting account will be at a
                                           derived address of STAKE_ACCOUNT_ADDRESS
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --stake-authority <KEYPAIR>        Authorized staker [default: cli config keypair]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <STAKE_ACCOUNT_ADDRESS>    Stake account to be deactivated (or base of derived address if --seed is used). , one
                               of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-decode-transaction
```text
solana-decode-transaction 
Decode a serialized transaction

USAGE:
    solana decode-transaction [FLAGS] [OPTIONS] <TRANSACTION> <ENCODING>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <TRANSACTION>    transaction to decode
    <ENCODING>       transaction encoding [default: base58]  [possible values: base58, base64]
```

#### solana-delegate-stake
```text
solana-delegate-stake 
Delegate stake to a vote account

USAGE:
    solana delegate-stake [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS> <VOTE_ACCOUNT_ADDRESS>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --stake-authority <KEYPAIR>        Authorized staker [default: cli config keypair]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <STAKE_ACCOUNT_ADDRESS>    Stake account to delegate, one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
    <VOTE_ACCOUNT_ADDRESS>     The vote account to which the stake will be delegated, one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-epoch
```text
solana-epoch 
Get current epoch

USAGE:
    solana epoch [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-epoch-info
```text
solana-epoch-info 
Get information about the current epoch

USAGE:
    solana epoch-info [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-feature
```text
solana-feature 
Runtime feature management

USAGE:
    solana feature [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

SUBCOMMANDS:
    activate    Activate a runtime feature
    help        Prints this message or the help of the given subcommand(s)
    status      Query runtime feature status
```

#### solana-fees
```text
solana-fees 
Display current cluster fees (Deprecated in v1.8.0)

USAGE:
    solana fees [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Query fees for BLOCKHASH instead of the the most recent blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-first-available-block
```text
solana-first-available-block 
Get the first available block in the storage

USAGE:
    solana first-available-block [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-genesis-hash
```text
solana-genesis-hash 
Get the genesis hash

USAGE:
    solana genesis-hash [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-gossip
```text
solana-gossip 
Show the current gossip network nodes

USAGE:
    solana gossip [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-help
```text
solana-help 
Prints this message or the help of the given subcommand(s)

USAGE:
    solana help [subcommand]...

ARGS:
    <subcommand>...    The subcommand whose help message to display
```

#### solana-inflation
```text
solana-inflation 
Show inflation information

USAGE:
    solana inflation [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    rewards    Show inflation rewards for a set of addresses
```

#### solana-largest-accounts
```text
solana-largest-accounts 
Get addresses of largest cluster accounts

USAGE:
    solana largest-accounts [FLAGS] [OPTIONS]

FLAGS:
        --circulating                    Filter address list to only circulating accounts
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --non-circulating                Filter address list to only non-circulating accounts
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-leader-schedule
```text
solana-leader-schedule 
Display leader schedule

USAGE:
    solana leader-schedule [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --epoch <EPOCH>                    Epoch to show leader schedule for. [default: current]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-live-slots
```text
solana-live-slots 
Show information about the current slot progression

USAGE:
    solana live-slots [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-logs
```text
solana-logs 
Stream transaction logs

USAGE:
    solana logs [FLAGS] [OPTIONS] [ADDRESS]

FLAGS:
    -h, --help                           Prints help information
        --include-votes                  Include vote transactions when monitoring all transactions
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <ADDRESS>    Account address to monitor [default: monitor all transactions except for votes] , one of:
                   * a base58-encoded public key
                   * a path to a keypair file
                   * a hyphen; signals a JSON-encoded keypair on stdin
                   * the 'ASK' keyword; to recover a keypair via its seed phrase
                   * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-merge-stake
```text
solana-merge-stake 
Merges one stake account into another

USAGE:
    solana merge-stake [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS> <SOURCE_STAKE_ACCOUNT_ADDRESS>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --stake-authority <KEYPAIR>        Authorized staker [default: cli config keypair]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <STAKE_ACCOUNT_ADDRESS>           Stake account to merge into, one of:
                                        * a base58-encoded public key
                                        * a path to a keypair file
                                        * a hyphen; signals a JSON-encoded keypair on stdin
                                        * the 'ASK' keyword; to recover a keypair via its seed phrase
                                        * a hardware wallet keypair URL (i.e. usb://ledger)
    <SOURCE_STAKE_ACCOUNT_ADDRESS>    Source stake account for the merge.  If successful, this stake account will no
                                      longer exist after the merge, one of:
                                        * a base58-encoded public key
                                        * a path to a keypair file
                                        * a hyphen; signals a JSON-encoded keypair on stdin
                                        * the 'ASK' keyword; to recover a keypair via its seed phrase
                                        * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-new-nonce
```text
solana-new-nonce 
Generate a new nonce, rendering the existing nonce useless

USAGE:
    solana new-nonce [FLAGS] [OPTIONS] <NONCE_ACCOUNT_ADDRESS>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <NONCE_ACCOUNT_ADDRESS>    Address of the nonce account. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-nonce
```text
solana-nonce 
Get the current nonce value

USAGE:
    solana nonce [FLAGS] [OPTIONS] <NONCE_ACCOUNT_ADDRESS>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <NONCE_ACCOUNT_ADDRESS>    Address of the nonce account to display. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-nonce-account
```text
solana-nonce-account 
Show the contents of a nonce account

USAGE:
    solana nonce-account [FLAGS] [OPTIONS] <NONCE_ACCOUNT_ADDRESS>

FLAGS:
    -h, --help                           Prints help information
        --lamports                       Display balance in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <NONCE_ACCOUNT_ADDRESS>    Address of the nonce account to display. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-ping
```text
solana-ping 
Submit transactions sequentially

USAGE:
    solana ping [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
    -D, --print-timestamp                Print timestamp (unix time + microseconds as in gettimeofday) before each line
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>                  Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>
            Return information at the selected commitment level [possible values: processed, confirmed, finalized]

        --compute-unit-price <MICRO-SPARKS>    Set the price in micro-sparks of each transaction compute unit
    -C, --config <FILEPATH>
            Configuration file to use [default: ~/.config/solana/cli/config.yml]

    -c, --count <NUMBER>                         Stop after submitting count transactions
    -i, --interval <SECONDS>
            Wait interval seconds between submitting the next transaction [default: 2]

    -u, --url <URL_OR_MONIKER>
             URL for cluster's JSON RPC.

    -k, --keypair <KEYPAIR>                      Filepath or URL to a keypair
        --output <FORMAT>
            Return information in specified output format [possible values: json, json-compact]

    -t, --timeout <SECONDS>                      Wait up to timeout seconds for transaction confirmation [default: 15]
        --ws <URL>                               WebSocket URL for the cluster
```

#### solana-program
```text
solana-program 
Program management

USAGE:
    solana program [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

SUBCOMMANDS:
    close                    Close a program or buffer account and withdraw all sparks
    deploy                   Deploy a program
    dump                     Write the program data to a file
    help                     Prints this message or the help of the given subcommand(s)
    set-buffer-authority     Set a new buffer authority
    set-upgrade-authority    Set a new program authority
    show                     Display information about a buffer or program
    write-buffer             Writes a program into a buffer account
```

#### solana-rent
```text
solana-rent 
Calculate per-epoch and rent-exempt-minimum values for a given account data field length.

USAGE:
    solana rent [FLAGS] [OPTIONS] <DATA_LENGTH_OR_MONIKER>

FLAGS:
    -h, --help                           Prints help information
        --lamports                       Display rent in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <DATA_LENGTH_OR_MONIKER>    Length of data field in the account to calculate rent for, or moniker: [nonce,
                                stake, system, vote]
```

#### solana-resolve-signer
```text
solana-resolve-signer 
Checks that a signer is valid, and returns its specific path; useful for signers that may be specified generally, eg.
usb://ledger

USAGE:
    solana resolve-signer [FLAGS] [OPTIONS] <SIGNER_KEYPAIR>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <SIGNER_KEYPAIR>    The signer path to resolve
```

#### solana-slot
```text
solana-slot 
Get current slot

USAGE:
    solana slot [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-split-stake
```text
solana-split-stake 
Duplicate a stake account, splitting the tokens between the two

USAGE:
    solana split-stake [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS> <SPLIT_STAKE_ACCOUNT> <AMOUNT>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --seed <STRING>                    Seed for address generation; if specified, the resulting account will be at a
                                           derived address of SPLIT_STAKE_ACCOUNT
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --stake-authority <KEYPAIR>        Authorized staker [default: cli config keypair]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <STAKE_ACCOUNT_ADDRESS>    Stake account to split (or base of derived address if --seed is used). , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
    <SPLIT_STAKE_ACCOUNT>      Keypair of the new stake account
    <AMOUNT>                   The amount to move into the new stake account, in POWR
```

#### solana-stake-account
```text
solana-stake-account 
Show the contents of a stake account

USAGE:
    solana stake-account [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS>

FLAGS:
    -h, --help                           Prints help information
        --lamports                       Display balance in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information
        --with-rewards                   Display inflation rewards

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --num-rewards-epochs <NUM>         Display rewards for NUM recent epochs, max 10 [default: latest epoch only]
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <STAKE_ACCOUNT_ADDRESS>    The stake account to display. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-stake-authorize
```text
solana-stake-authorize 
Authorize a new signing keypair for the given stake account

USAGE:
    solana stake-authorize [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS> --new-stake-authority <PUBKEY> --new-withdraw-authority <PUBKEY>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --no-wait                        Return signature immediately after submitting the transaction, instead of
                                         waiting for confirmations
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>              Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>      Return information at the selected commitment level [possible values:
                                             processed, confirmed, finalized]
    -C, --config <FILEPATH>                  Configuration file to use [default:
                                             ~/.config/solana/cli/config.yml]
        --custodian <KEYPAIR>                Authority to override account lockup
        --fee-payer <KEYPAIR>                Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                             or the pubkey of an offline signer, provided an appropriate --signer
                                             argument 
                                             is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>               URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                  Filepath or URL to a keypair
        --with-memo <MEMO>                   Specify a memo string to include in the transaction.
        --new-stake-authority <PUBKEY>       New authorized staker, one of:
                                               * a base58-encoded public key
                                               * a path to a keypair file
                                               * a hyphen; signals a JSON-encoded keypair on stdin
                                               * the 'ASK' keyword; to recover a keypair via its seed phrase
                                               * a hardware wallet keypair URL (i.e. usb://ledger)
        --new-withdraw-authority <PUBKEY>    New authorized withdrawer. , one of:
                                               * a base58-encoded public key
                                               * a path to a keypair file
                                               * a hyphen; signals a JSON-encoded keypair on stdin
                                               * the 'ASK' keyword; to recover a keypair via its seed phrase
                                               * a hardware wallet keypair URL (i.e. usb://ledger)
        --nonce <PUBKEY>                     Provide the nonce account to use when creating a nonced 
                                             transaction. Nonced transactions are useful when a transaction 
                                             requires a lengthy signing process.
        --nonce-authority <KEYPAIR>          Provide the nonce authority keypair to use when signing a nonced
                                             transaction
        --output <FORMAT>                    Return information in specified output format [possible values: json, json-
                                             compact]
        --signer <PUBKEY=SIGNATURE>...       Provide a public-key/signature pair for the transaction
        --stake-authority <KEYPAIR>          Authorized staker [default: cli config keypair]
        --ws <URL>                           WebSocket URL for the cluster
        --withdraw-authority <KEYPAIR>       Authorized withdrawer [default: cli config keypair]

ARGS:
    <STAKE_ACCOUNT_ADDRESS>    Stake account in which to set a new authority. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-stake-authorize-checked
```text
solana-stake-authorize-checked 
Authorize a new signing keypair for the given stake account, checking the authority as a signer

USAGE:
    solana stake-authorize-checked [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --no-wait                        Return signature immediately after submitting the transaction, instead of
                                         waiting for confirmations
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>               Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>       Return information at the selected commitment level [possible values:
                                              processed, confirmed, finalized]
    -C, --config <FILEPATH>                   Configuration file to use [default:
                                              ~/.config/solana/cli/config.yml]
        --custodian <KEYPAIR>                 Authority to override account lockup
        --fee-payer <KEYPAIR>                 Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                              or the pubkey of an offline signer, provided an appropriate --signer
                                              argument 
                                              is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>                URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                   Filepath or URL to a keypair
        --with-memo <MEMO>                    Specify a memo string to include in the transaction.
        --new-stake-authority <KEYPAIR>       New authorized staker
        --new-withdraw-authority <KEYPAIR>    New authorized withdrawer
        --nonce <PUBKEY>                      Provide the nonce account to use when creating a nonced 
                                              transaction. Nonced transactions are useful when a transaction 
                                              requires a lengthy signing process.
        --nonce-authority <KEYPAIR>           Provide the nonce authority keypair to use when signing a nonced
                                              transaction
        --output <FORMAT>                     Return information in specified output format [possible values: json,
                                              json-compact]
        --signer <PUBKEY=SIGNATURE>...        Provide a public-key/signature pair for the transaction
        --stake-authority <KEYPAIR>           Authorized staker [default: cli config keypair]
        --ws <URL>                            WebSocket URL for the cluster
        --withdraw-authority <KEYPAIR>        Authorized withdrawer [default: cli config keypair]

ARGS:
    <STAKE_ACCOUNT_ADDRESS>    Stake account in which to set a new authority. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-stake-history
```text
solana-stake-history 
Show the stake history

USAGE:
    solana stake-history [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --lamports                       Display balance in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --limit <NUM>                      Display NUM recent epochs worth of stake history in text mode. 0 for all
                                           [default: 10]
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-stake-set-lockup
```text
solana-stake-set-lockup 
Set Lockup for the stake account

USAGE:
    solana stake-set-lockup [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS> <--lockup-epoch <NUMBER>|--lockup-date <RFC3339 DATETIME>|--new-custodian <PUBKEY>>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>             Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>     Return information at the selected commitment level [possible values:
                                            processed, confirmed, finalized]
    -C, --config <FILEPATH>                 Configuration file to use [default:
                                            ~/.config/solana/cli/config.yml]
        --custodian <KEYPAIR>               Keypair of the existing custodian [default: cli config pubkey]
        --fee-payer <KEYPAIR>               Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                            or the pubkey of an offline signer, provided an appropriate --signer
                                            argument 
                                            is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>              URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                 Filepath or URL to a keypair
        --lockup-date <RFC3339 DATETIME>    The date and time at which this account will be available for withdrawal
        --lockup-epoch <NUMBER>             The epoch height at which this account will be available for withdrawal
        --with-memo <MEMO>                  Specify a memo string to include in the transaction.
        --new-custodian <PUBKEY>            Identity of a new lockup custodian. , one of:
                                              * a base58-encoded public key
                                              * a path to a keypair file
                                              * a hyphen; signals a JSON-encoded keypair on stdin
                                              * the 'ASK' keyword; to recover a keypair via its seed phrase
                                              * a hardware wallet keypair URL (i.e. usb://ledger)
        --nonce <PUBKEY>                    Provide the nonce account to use when creating a nonced 
                                            transaction. Nonced transactions are useful when a transaction 
                                            requires a lengthy signing process.
        --nonce-authority <KEYPAIR>         Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                   Return information in specified output format [possible values: json, json-
                                            compact]
        --signer <PUBKEY=SIGNATURE>...      Provide a public-key/signature pair for the transaction
        --ws <URL>                          WebSocket URL for the cluster

ARGS:
    <STAKE_ACCOUNT_ADDRESS>    Stake account for which to set lockup parameters. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-stake-set-lockup-checked
```text
solana-stake-set-lockup-checked 
Set Lockup for the stake account, checking the new authority as a signer

USAGE:
    solana stake-set-lockup-checked [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS> <--lockup-epoch <NUMBER>|--lockup-date <RFC3339 DATETIME>|--new-custodian <KEYPAIR>>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>             Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>     Return information at the selected commitment level [possible values:
                                            processed, confirmed, finalized]
    -C, --config <FILEPATH>                 Configuration file to use [default:
                                            ~/.config/solana/cli/config.yml]
        --custodian <KEYPAIR>               Keypair of the existing custodian [default: cli config pubkey]
        --fee-payer <KEYPAIR>               Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                            or the pubkey of an offline signer, provided an appropriate --signer
                                            argument 
                                            is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>              URL for cluster's JSON RPC
    -k, --keypair <KEYPAIR>                 Filepath or URL to a keypair
        --lockup-date <RFC3339 DATETIME>    The date and time at which this account will be available for withdrawal
        --lockup-epoch <NUMBER>             The epoch height at which this account will be available for withdrawal
        --with-memo <MEMO>                  Specify a memo string to include in the transaction.
        --new-custodian <KEYPAIR>           Keypair of a new lockup custodian
        --nonce <PUBKEY>                    Provide the nonce account to use when creating a nonced 
                                            transaction. Nonced transactions are useful when a transaction 
                                            requires a lengthy signing process.
        --nonce-authority <KEYPAIR>         Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                   Return information in specified output format [possible values: json, json-
                                            compact]
        --signer <PUBKEY=SIGNATURE>...      Provide a public-key/signature pair for the transaction
        --ws <URL>                          WebSocket URL for the cluster

ARGS:
    <STAKE_ACCOUNT_ADDRESS>    Stake account for which to set lockup parameters. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-stakes
```text
solana-stakes 
Show stake account information

USAGE:
    solana stakes [FLAGS] [OPTIONS] [VOTE_ACCOUNT_PUBKEYS]...

FLAGS:
    -h, --help                           Prints help information
        --lamports                       Display balance in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
        --withdraw-authority <PUBKEY>      Only show stake accounts with the provided withdraw authority. , one of:
                                             * a base58-encoded public key
                                             * a path to a keypair file
                                             * a hyphen; signals a JSON-encoded keypair on stdin
                                             * the 'ASK' keyword; to recover a keypair via its seed phrase
                                             * a hardware wallet keypair URL (i.e. usb://ledger)

ARGS:
    <VOTE_ACCOUNT_PUBKEYS>...    Only show stake accounts delegated to the provided vote accounts. , one of:
                                   * a base58-encoded public key
                                   * a path to a keypair file
                                   * a hyphen; signals a JSON-encoded keypair on stdin
                                   * the 'ASK' keyword; to recover a keypair via its seed phrase
                                   * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-supply
```text
solana-supply 
Get information about the cluster supply of POWR

USAGE:
    solana supply [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --print-accounts                 Print list of non-circualting account addresses
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-transaction-count
```text
solana-transaction-count 
Get current transaction count

USAGE:
    solana transaction-count [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster
```

#### solana-transaction-history
```text
solana-transaction-history 
Show historical transactions affecting the given address from newest to oldest

USAGE:
    solana transaction-history [FLAGS] [OPTIONS] <ADDRESS>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --show-transactions              Display the full transactions
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --before <TRANSACTION_SIGNATURE>    Start with the first signature older than this one
        --commitment <COMMITMENT_LEVEL>     Return information at the selected commitment level [possible values:
                                            processed, confirmed, finalized]
    -C, --config <FILEPATH>                 Configuration file to use [default:
                                            ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>              URL for cluster's JSON RPC
    -k, --keypair <KEYPAIR>                 Filepath or URL to a keypair
        --limit <LIMIT>                     Maximum number of transaction signatures to return [default: 1000]
        --output <FORMAT>                   Return information in specified output format [possible values: json, json-
                                            compact]
        --ws <URL>                          WebSocket URL for the cluster

ARGS:
    <ADDRESS>    Account address, one of:
                   * a base58-encoded public key
                   * a path to a keypair file
                   * a hyphen; signals a JSON-encoded keypair on stdin
                   * the 'ASK' keyword; to recover a keypair via its seed phrase
                   * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-transfer
```text
solana-transfer 
Transfer funds between system accounts

USAGE:
    solana transfer [FLAGS] [OPTIONS] <RECIPIENT_ADDRESS> <AMOUNT>

FLAGS:
        --allow-unfunded-recipient       Complete the transfer even if the recipient address is not funded
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --no-wait                        Return signature immediately after submitting the transaction, instead of
                                         waiting for confirmations
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
        --from <FROM_ADDRESS>              Source account of funds (if different from client local account). , one of:
                                             * a base58-encoded public key
                                             * a path to a keypair file
                                             * a hyphen; signals a JSON-encoded keypair on stdin
                                             * the 'ASK' keyword; to recover a keypair via its seed phrase
                                             * a hardware wallet keypair URL (i.e. usb://ledger)
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <RECIPIENT_ADDRESS>    The account address of recipient. , one of:
                             * a base58-encoded public key
                             * a path to a keypair file
                             * a hyphen; signals a JSON-encoded keypair on stdin
                             * the 'ASK' keyword; to recover a keypair via its seed phrase
                             * a hardware wallet keypair URL (i.e. usb://ledger)
    <AMOUNT>               The amount to send, in POWR; accepts keyword ALL
```

#### solana-upgrade-nonce-account
```text
solana-upgrade-nonce-account 
One-time idempotent upgrade of legacy nonce versions in order to bump them out of chain blockhash domain.

USAGE:
    solana upgrade-nonce-account [FLAGS] [OPTIONS] <NONCE_ACCOUNT_ADDRESS>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <NONCE_ACCOUNT_ADDRESS>    Nonce account to upgrade. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-validator-info
```text
solana-validator-info 
Publish/get Validator info on Powerledger blockchain

USAGE:
    solana validator-info [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

SUBCOMMANDS:
    get        Get and parse Powerledger blockchain Validator info
    help       Prints this message or the help of the given subcommand(s)
    publish    Publish Validator info on Powerledger blockchain
```

#### solana-validators
```text
solana-validators 
Show summary information about the current validators

USAGE:
    solana validators [FLAGS] [OPTIONS]

FLAGS:
    -h, --help                           Prints help information
        --keep-unstaked-delinquents      Don't discard unstaked, delinquent validators
        --lamports                       Display balance in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
    -n, --number                         Number the validators
    -r, --reverse                        Reverse order while sorting
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>
            Return information at the selected commitment level [possible values: processed, confirmed, finalized]

    -C, --config <FILEPATH>
            Configuration file to use [default: ~/.config/solana/cli/config.yml]

        --delinquent-slot-distance <SLOT_DISTANCE>
            Minimum slot distance from the tip to consider a validator delinquent. [default: 128]

    -u, --url <URL_OR_MONIKER>
             URL for cluster's JSON RPC.

    -k, --keypair <KEYPAIR>                           Filepath or URL to a keypair
        --output <FORMAT>
            Return information in specified output format [possible values: json, json-compact]

        --sort <sort>
            Sort order (does not affect JSON output) [default: stake]  [possible values: delinquent, commission,
            credits, identity, last-vote, root, skip-rate, stake, version, vote-account]
        --ws <URL>                                    WebSocket URL for the cluster
```

#### solana-vote-account
```text
solana-vote-account 
Show the contents of a vote account

USAGE:
    solana vote-account [FLAGS] [OPTIONS] <VOTE_ACCOUNT_ADDRESS>

FLAGS:
    -h, --help                           Prints help information
        --lamports                       Display balance in sparks instead of POWR
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information
        --with-rewards                   Display inflation rewards

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --num-rewards-epochs <NUM>         Display rewards for NUM recent epochs, max 10 [default: latest epoch only]
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <VOTE_ACCOUNT_ADDRESS>    Vote account pubkey. , one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-vote-authorize-voter
```text
solana-vote-authorize-voter 
Authorize a new vote signing keypair for the given vote account

USAGE:
    solana vote-authorize-voter [FLAGS] [OPTIONS] <VOTE_ACCOUNT_ADDRESS> <AUTHORIZED_KEYPAIR> <NEW_AUTHORIZED_PUBKEY>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <VOTE_ACCOUNT_ADDRESS>     Vote account in which to set the authorized voter. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
    <AUTHORIZED_KEYPAIR>       Current authorized vote signer.
    <NEW_AUTHORIZED_PUBKEY>    New authorized vote signer. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-vote-authorize-voter-checked
```text
solana-vote-authorize-voter-checked 
Authorize a new vote signing keypair for the given vote account, checking the new authority as a signer

USAGE:
    solana vote-authorize-voter-checked [FLAGS] [OPTIONS] <VOTE_ACCOUNT_ADDRESS> <AUTHORIZED_KEYPAIR> <NEW_AUTHORIZED_KEYPAIR>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <VOTE_ACCOUNT_ADDRESS>      Vote account in which to set the authorized voter. , one of:
                                  * a base58-encoded public key
                                  * a path to a keypair file
                                  * a hyphen; signals a JSON-encoded keypair on stdin
                                  * the 'ASK' keyword; to recover a keypair via its seed phrase
                                  * a hardware wallet keypair URL (i.e. usb://ledger)
    <AUTHORIZED_KEYPAIR>        Current authorized vote signer.
    <NEW_AUTHORIZED_KEYPAIR>    New authorized vote signer.
```

#### solana-vote-authorize-withdrawer
```text
solana-vote-authorize-withdrawer 
Authorize a new withdraw signing keypair for the given vote account

USAGE:
    solana vote-authorize-withdrawer [FLAGS] [OPTIONS] <VOTE_ACCOUNT_ADDRESS> <AUTHORIZED_KEYPAIR> <AUTHORIZED_PUBKEY>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <VOTE_ACCOUNT_ADDRESS>    Vote account in which to set the authorized withdrawer. , one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
    <AUTHORIZED_KEYPAIR>      Current authorized withdrawer.
    <AUTHORIZED_PUBKEY>       New authorized withdrawer. , one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
```

#### solana-vote-authorize-withdrawer-checked
```text
solana-vote-authorize-withdrawer-checked 
Authorize a new withdraw signing keypair for the given vote account, checking the new authority as a signer

USAGE:
    solana vote-authorize-withdrawer-checked [FLAGS] [OPTIONS] <VOTE_ACCOUNT_ADDRESS> <AUTHORIZED_KEYPAIR> <NEW_AUTHORIZED_KEYPAIR>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <VOTE_ACCOUNT_ADDRESS>      Vote account in which to set the authorized withdrawer. , one of:
                                  * a base58-encoded public key
                                  * a path to a keypair file
                                  * a hyphen; signals a JSON-encoded keypair on stdin
                                  * the 'ASK' keyword; to recover a keypair via its seed phrase
                                  * a hardware wallet keypair URL (i.e. usb://ledger)
    <AUTHORIZED_KEYPAIR>        Current authorized withdrawer.
    <NEW_AUTHORIZED_KEYPAIR>    New authorized withdrawer.
```

#### solana-vote-update-commission
```text
solana-vote-update-commission 
Update the vote account's commission

USAGE:
    solana vote-update-commission [FLAGS] [OPTIONS] <VOTE_ACCOUNT_ADDRESS> <PERCENTAGE> <AUTHORIZED_KEYPAIR>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <VOTE_ACCOUNT_ADDRESS>    Vote account to update. , one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
    <PERCENTAGE>              The new commission
    <AUTHORIZED_KEYPAIR>      Authorized withdrawer keypair
```

#### solana-vote-update-validator
```text
solana-vote-update-validator 
Update the vote account's validator identity

USAGE:
    solana vote-update-validator [FLAGS] [OPTIONS] <VOTE_ACCOUNT_ADDRESS> <IDENTITY_KEYPAIR> <AUTHORIZED_KEYPAIR>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <VOTE_ACCOUNT_ADDRESS>    Vote account to update. , one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
    <IDENTITY_KEYPAIR>        Keypair of new validator that will vote with this account
    <AUTHORIZED_KEYPAIR>      Authorized withdrawer keypair
```

#### solana-wait-for-max-stake
```text
solana-wait-for-max-stake 
Wait for the max stake of any one node to drop below a percentage of total.

USAGE:
    solana wait-for-max-stake [FLAGS] [OPTIONS] [PERCENT]

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <PERCENT>    
```

#### solana-withdraw-from-nonce-account
```text
solana-withdraw-from-nonce-account 
Withdraw POWR from the nonce account

USAGE:
    solana withdraw-from-nonce-account [FLAGS] [OPTIONS] <NONCE_ACCOUNT_ADDRESS> <RECIPIENT_ADDRESS> <AMOUNT>

FLAGS:
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --ws <URL>                         WebSocket URL for the cluster

ARGS:
    <NONCE_ACCOUNT_ADDRESS>    Nonce account to withdraw from. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
    <RECIPIENT_ADDRESS>        The account to which the POWR should be transferred. , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
    <AMOUNT>                   The amount to withdraw from the nonce account, in POWR
```

#### solana-withdraw-from-vote-account
```text
solana-withdraw-from-vote-account 
Withdraw POWR from a vote account into a specified account

USAGE:
    solana withdraw-from-vote-account [FLAGS] [OPTIONS] <VOTE_ACCOUNT_ADDRESS> <RECIPIENT_ADDRESS> <AMOUNT>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --authorized-withdrawer <AUTHORIZED_KEYPAIR>    Authorized withdrawer [default: cli config keypair]
        --blockhash <BLOCKHASH>                         Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>
            Return information at the selected commitment level [possible values: processed, confirmed, finalized]

    -C, --config <FILEPATH>
            Configuration file to use [default: ~/.config/solana/cli/config.yml]

        --fee-payer <KEYPAIR>
            Specify the fee-payer account. This may be a keypair file, the ASK keyword 
            or the pubkey of an offline signer, provided an appropriate --signer argument 
            is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>
             URL for cluster's JSON RPC.

    -k, --keypair <KEYPAIR>                             Filepath or URL to a keypair
        --with-memo <MEMO>                              Specify a memo string to include in the transaction.
        --nonce <PUBKEY>
            Provide the nonce account to use when creating a nonced 
            transaction. Nonced transactions are useful when a transaction 
            requires a lengthy signing process.
        --nonce-authority <KEYPAIR>
            Provide the nonce authority keypair to use when signing a nonced transaction

        --output <FORMAT>
            Return information in specified output format [possible values: json, json-compact]

        --signer <PUBKEY=SIGNATURE>...                  Provide a public-key/signature pair for the transaction
        --ws <URL>                                      WebSocket URL for the cluster

ARGS:
    <VOTE_ACCOUNT_ADDRESS>    Vote account from which to withdraw. , one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
    <RECIPIENT_ADDRESS>       The recipient of withdrawn POWR. , one of:
                                * a base58-encoded public key
                                * a path to a keypair file
                                * a hyphen; signals a JSON-encoded keypair on stdin
                                * the 'ASK' keyword; to recover a keypair via its seed phrase
                                * a hardware wallet keypair URL (i.e. usb://ledger)
    <AMOUNT>                  The amount to withdraw, in POWR; accepts keyword ALL, which for this command means
                              account balance minus rent-exempt minimum
```

#### solana-withdraw-stake
```text
solana-withdraw-stake 
Withdraw the unstaked POWR from the stake account

USAGE:
    solana withdraw-stake [FLAGS] [OPTIONS] <STAKE_ACCOUNT_ADDRESS> <RECIPIENT_ADDRESS> <AMOUNT>

FLAGS:
        --dump-transaction-message       Display the base64 encoded binary transaction message in sign-only mode
    -h, --help                           Prints help information
        --no-address-labels              Do not use address labels in the output
        --sign-only                      Sign the transaction offline
        --skip-seed-phrase-validation    Skip validation of seed phrases. Use this if your phrase does not use the BIP39
                                         official English word list
    -V, --version                        Prints version information
    -v, --verbose                        Show additional information

OPTIONS:
        --blockhash <BLOCKHASH>            Use the supplied blockhash
        --commitment <COMMITMENT_LEVEL>    Return information at the selected commitment level [possible values:
                                           processed, confirmed, finalized]
    -C, --config <FILEPATH>                Configuration file to use [default:
                                           ~/.config/solana/cli/config.yml]
        --custodian <KEYPAIR>              Authority to override account lockup
        --fee-payer <KEYPAIR>              Specify the fee-payer account. This may be a keypair file, the ASK keyword 
                                           or the pubkey of an offline signer, provided an appropriate --signer argument 
                                           is also passed. Defaults to the client keypair.
    -u, --url <URL_OR_MONIKER>             URL for cluster's JSON RPC.
    -k, --keypair <KEYPAIR>                Filepath or URL to a keypair
        --with-memo <MEMO>                 Specify a memo string to include in the transaction.
        --nonce <PUBKEY>                   Provide the nonce account to use when creating a nonced 
                                           transaction. Nonced transactions are useful when a transaction 
                                           requires a lengthy signing process. 
        --nonce-authority <KEYPAIR>        Provide the nonce authority keypair to use when signing a nonced transaction
        --output <FORMAT>                  Return information in specified output format [possible values: json, json-
                                           compact]
        --seed <STRING>                    Seed for address generation; if specified, the resulting account will be at a
                                           derived address of STAKE_ACCOUNT_ADDRESS
        --signer <PUBKEY=SIGNATURE>...     Provide a public-key/signature pair for the transaction
        --ws <URL>                         WebSocket URL for the cluster
        --withdraw-authority <KEYPAIR>     Authorized withdrawer [default: cli config keypair]

ARGS:
    <STAKE_ACCOUNT_ADDRESS>    Stake account from which to withdraw (or base of derived address if --seed is used).
                               , one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
    <RECIPIENT_ADDRESS>        Recipient of withdrawn POWR, one of:
                                 * a base58-encoded public key
                                 * a path to a keypair file
                                 * a hyphen; signals a JSON-encoded keypair on stdin
                                 * the 'ASK' keyword; to recover a keypair via its seed phrase
                                 * a hardware wallet keypair URL (i.e. usb://ledger)
    <AMOUNT>                   The amount to withdraw from the stake account, in POWR; accepts keyword ALL
```

