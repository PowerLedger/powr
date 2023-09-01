<p align="center">
  <a href="https://powerledger.io">
    <img alt="powr" src="https://github.com/PowerLedger/powr/raw/master/logo.png" width="250" />
  </a>
</p>

# POWERLEDGER ENERGY BLOCKCHAIN

## IMPORTANT NOTICE - PLEASE READ BEFORE PROCEEDING

Use of this software and access to the Powerledger Blockchain is subject to the Powerledger Validator
Node Terms and Conditions set out in the Terms and Conditions file.

By downloading, installing and using the Powerledger Node Software and/or connecting to the Powerledger
Blockchain you agree to the Terms and Conditions set out in the Terms and Conditions.

All software is Copyright © 2023 Power Ledger Pty Ltd unless indicated otherwise.  Unauthorised use
prohibited. All Rights Reserved.

## powr validator node configuration

### Min Requirements for mainnet validator

```
* 12 Cores, 64 GB RAM
* ┬ 3 separate storage devices:
  ├*- 250 GB of storage for the operating system
  ├*- 1 TB of storage for the accounts folder
  └*┬ 1 TB of storage for the ledger folder (pruned nodes) , OR , 
    └ 4 TB of storage for the ledger folder (full history nodes). 
* Rust 1.59 installed
```

### Download the project

```bash
# Git pull powr and powr-program-library repositories
# Both repos need to be downloaded in the same location
git clone https://github.com/powerledger/powr
git clone https://github.com/powerledger/powr-program-library
```

## Compile the code

```bash
cd powr
cargo clean
cd scripts
./cargo-install-all.sh /home/validator-admin/.local/share/powr
```

## Append to .bashrc

```bash
echo -e "\nexport PATH=\$PATH:/home/validator-admin/.local/share/powr/bin" >> ~/.bashrc
cd
. .bashrc
```

## Create identities

At least 2 keypairs are necessary, and IDENTITY keypair and a VOTE-ACOUNT keypair.

We suggest creating a third keypair that is going to function as the withdrawal authority
for the vote account.

```bash
# Keypair generation example: 

mkdir ~/keypairs
solana-keygen new --word-count 24 --no-bip39-passphrase -o ~/keypairs/id.json
solana-keygen new --word-count 24 --no-bip39-passphrase -o ~/keypairs/vote-account.json
solana-keygen new --word-count 24 --no-bip39-passphrase -o ~/keypairs/withdraw-account.json
chmod 400 ~/keypairs/*
```

## Create config file

Execute the following commands to create the config file:

```bash

```bash
[ ! -d "~/.config/solana/cli" ] && mkdir -p ~/.config/solana/cli
cat > ~/.config/solana/cli/config.yml <<EOF
---
json_rpc_url: "https://powr-api.mainnet.powerledger.io"
websocket_url: ""
keypair_path: $HOME/keypairs/id.json
address_labels:
  "11111111111111111111111111111111": SystemProgram
commitment: confirmed
EOF
```

If your IDENTITY keypair is located in a different folder, it can be changed using the
following command:

```bash
solana config set -k /PATH_TO_KEYPAIR/id.json
```

## Create vote account

Example usage:

```bash
solana create-vote-account /PATH_TO_KEYPAIR/vote-account.json /PATH_TO_KEYPAIR/id.json $(solana address -k /PATH_TO_KEYPAIR/withdraw-account.json) --commission <0-100>
```

If the commission needs to be updated:

```bash
solana vote-update-commission $(solana address -k /PATH_TO_KEYPAIR/vote-account.json) <0-100> /PATH_TO_KEYPAIR/withdraw-account.json
```

## Starting you validator node

IMPORTANT NOTE: You can modify the command to fit your requirements
but the list of --known-validator(s) must be provided.

### Full history node

```bash
#!/usr/bin/env bash

exec <PATH TO>/solana-validator \
--ledger <PATH TO>/ledger \
--accounts <PATH TO>/accounts \
--full-rpc-api \
--private-rpc \
--rpc-port 8899 \
--snapshot-interval-slots 200 \
--account-index program-id \
--account-index spl-token-mint \
--account-index spl-token-owner \
--identity <PATH TO>/keypairs/id.json \
--authorized-voter <PATH TO>/keypairs/id.json \
--vote-account <PATH TO>/keypairs/vote-account.json \
--expected-genesis-hash B1xJgyn5UyWw2itxwwctHL5fLxY9ZbnsDf7AG2WXbpGY \
--known-validator 8cDZLH2Ha3v56WLcpQkH7U211uokkb6YbZ3Sc6r6WrUa \
--known-validator 6YDqUUE3JBA9sVLNNiWYQR7moAPyNfDsBKPEvGcuEfQ2 \
--known-validator 8oATwTdKJqUvWMkUjFDBVW23HDF5DbVRskxWCypXwRRB \
--known-validator 6H69egwLDXs27r7tfUu5oxesbcYbi6YL8JW2D22jkBEi \
--wal-recovery-mode skip_any_corrupted_record \
--enable-rpc-transaction-history \
--enable-cpi-and-log-storage \
--entrypoint powr-entrypoint-1.mainnet.powerledger.io:8001 \
--entrypoint powr-entrypoint-2.mainnet.powerledger.io:8001 \
--dynamic-port-range 8000-9000 \
--log <PATH TO>/validator.log
```

### Pruned Node

```bash
#!/usr/bin/env bash

exec <PATH TO powr/target/release>/solana-validator \
--ledger <PATH TO>/ledger \
--accounts <PATH TO>/accounts \
--full-rpc-api \
--private-rpc \
--rpc-port 8899 \
--account-index program-id \
--account-index spl-token-mint \
--account-index spl-token-owner \
--identity <PATH TO>/keypairs/id.json \
--authorized-voter <PATH TO>/keypairs/id.json \
--vote-account <PATH TO>/keypairs/vote-account.json \
--expected-genesis-hash B1xJgyn5UyWw2itxwwctHL5fLxY9ZbnsDf7AG2WXbpGY \
--known-validator 8cDZLH2Ha3v56WLcpQkH7U211uokkb6YbZ3Sc6r6WrUa \
--known-validator 6YDqUUE3JBA9sVLNNiWYQR7moAPyNfDsBKPEvGcuEfQ2 \
--known-validator 8oATwTdKJqUvWMkUjFDBVW23HDF5DbVRskxWCypXwRRB \
--known-validator 6H69egwLDXs27r7tfUu5oxesbcYbi6YL8JW2D22jkBEi \
--wal-recovery-mode skip_any_corrupted_record \
--enable-rpc-transaction-history \
--entrypoint powr-entrypoint-1.mainnet.powerledger.io:8001 \
--entrypoint powr-entrypoint-2.mainnet.powerledger.io:8001 \
--dynamic-port-range 8000-9000 \
--limit-ledger-size \
--log <PATH TO>/validator.log
```

We HIGHLY recommend to manage the process using linux's `systemctl` command:

```bash
sudo su
cat > /etc/systemd/system/powr.service <<EOF
[Unit]
Description=Powerledger Blockchain Validator Service
Requires=network-online.target
After=network.target

[Service]
LimitNOFILE=1000000
Type=simple
User=<USER>
WorkingDirectory=/home/<USER>
ExecStart=<PATH TO SOLANA VALIDATOR SCRIPT>
Restart=always
RestartSec=1

[Install]
WantedBy=multi-user.target
EOF
exit
```

## System tunning

```bash
# Increase UDP buffers
sudo bash -c "cat >/etc/sysctl.d/20-solana-udp-buffers.conf <<EOF
# Increase UDP buffer size
net.core.rmem_default = 134217728
net.core.rmem_max = 134217728
net.core.wmem_default = 134217728
net.core.wmem_max = 134217728
EOF"
sudo sysctl -p /etc/sysctl.d/20-solana-udp-buffers.conf
```

```bash
# Increased memory mapped files limit
sudo bash -c "cat >/etc/sysctl.d/20-solana-mmaps.conf <<EOF
# Increase memory mapped files limit
vm.max_map_count = 1000000
EOF"
sudo sysctl -p /etc/sysctl.d/20-solana-mmaps.conf
```

```bash
# Add the following line to the [Service] section of your systemd service file, if you use one
LimitNOFILE=1000000
```

```bash
# Increasing max opened files limit
sudo bash -c "cat >/etc/security/limits.d/90-solana-nofiles.conf <<EOF
# Increase process file descriptor count limit
* - nofile 1000000
EOF"
```

```bash
# Close all open sessions (log out then, in again)
# check new limit
ulimit -Sn
```

## Create logrotate configuration

```bash
sudo su
cat > /etc/logrotate.d/<NAME OF THE systemctl SERVICE>.conf <<EOF
/PATH_TO_LOGFILE/validator.log {
    rotate 7
    daily
    missingok
    postrotate
        systemctl kill -s USR1 <NAME OF THE systemctl SERVICE>.service
    endscript
}
EOF
systemctl restart logrotate.service
exit
```

## Withdrawing tokens from vote-account

```bash
solana withdraw-from-vote-account \
$(solana-keygen pubkey /PATH_TO_KEYPAIR/vote-account.json) \
<RECIPIENT ADDRESS> \
<AMOUNT> \
--withdraw-authority /PATH_TO_KEYPAIR/withdraw-account.json
```

## Publish additional information about the validator

```bash
solana validator-info publish -k /PATH_TO_KEYPAIR/id.json -w "<URL OF THE VALIDATOR>" "<VALIDATOR NAME>"
```

## Getting published validators metadata

```bash
solana validator-info get
```
