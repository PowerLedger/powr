---
title: Publishing Validator Info
---

You can publish your validator information to the Powerledger blockchain to be publicly visible to other users.

## Run command to get info

Run the CLI command to publish a validator info account:

```bash
solana validator-info publish --keypair ~/validator-keypair.json <VALIDATOR_INFO_ARGS> <VALIDATOR_NAME>
```

For details about optional fields for VALIDATOR_INFO_ARGS:

```bash
solana validator-info publish --help
```

## Example Commands

Example publish command:

```bash
solana validator-info publish "Elvis Validator" -n elvis -w "https://elvis-validates.com"
```

Example query command:

```bash
solana validator-info get
```

which outputs

```text
Validator info from 8WdJvDz6obhADdxpGCiJKZsDYwTLNEDFizayqziDc9ah
  Validator pubkey: 6dMH3u76qZ7XG4bVboVRnBHR2FfrxEqTTTyj4xmyDMWo
  Info: {"keybaseUsername":"elvis","name":"Elvis Validator","website":"https://elvis-validates.com"}
```