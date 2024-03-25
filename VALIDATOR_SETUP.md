# Powerledger Blockchain Node Management Guide

## Setup
The guide below is designed for usage on a fresh Ubuntu 20.04 installation. The software should be compatible with most linux versions, although support can only be offered for ubuntu based installations.

### Setup user account
First, switch to the root user and create a new user account and make them a superuser. This guide will assume the name of the user is `validator-admin`, although this can be changed
```bash
# Add user called validator-admin as a super user
sudo su
useradd validator-admin
usermod -aG sudo validator-admin
```

Next, we need to ensure the account can access superuser command without requiring inputing a password.
```bash
visudo
```
Copy the following to the end of the file and save and quit
```bash
validator-admin ALL=(ALL) NOPASSWD: ALL
```

### Installing prerequsite software
```bash
# Install build tools
apt-get update && apt-get install build-essential curl libssl-dev pkg-config
```

#### Installing rust
Before compiling and running the validator software, we must install rust as a prerequsite, which we will do through the `rustup` tool.

```bash
# Install rustup and rust 1.59.0
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup install 1.59.0
rustup default 1.59.0
```

Veryify that the correct version of rust was installed successfully:
```bash
rustc --version
```

#### switch to created user
```bash
su - validator-admin
```

#### Append the rust binary to your PATH in .bashrc
```bash
echo -e "\nexport PATH=\$PATH:/home/validator-admin/.cargo/bin" >> ~/.bashrc
cd
. .bashrc
```

### Installing validator software
After installing rust, we must clone the software from the github repository and build it.

#### Download the project

```bash
# Change to home directory
cd ~
# Git pull powr and powr-program-library repositories
# Both repos need to be downloaded in the same location
git clone https://github.com/powerledger/powr
git clone https://github.com/powerledger/powr-program-library
```

#### Compile the code

```bash
cd powr
cargo clean
cd scripts
./cargo-install-all.sh /home/validator-admin/.local/share/powr
```

#### Append to .bashrc

```bash
echo -e "\nexport PATH=\$PATH:/home/validator-admin/.local/share/powr/bin" >> ~/.bashrc
cd
. .bashrc
```

### Create identities

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

### Create config file

Execute the following commands to create the config file:

```bash
[ ! -d "~/.config/solana/cli" ] && mkdir -p ~/.config/solana/cli
cat > ~/.config/solana/cli/config.yml << 'EOF'
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
following command (you will need to change the default location in subsequent commands if so)

```bash
solana config set -k /PATH_TO_KEYPAIR/id.json
```


## Create vote account

To create a vote account is a necessary step to first set up your validator. This requires a small amount of native blockchain POWR tokens to use as a transaction fee. Until a blockchain bridge is deployed, contact powerledger at validator@powerledger.io to arrange a swap of POWR to recieve native tokens to continue the steps below.

Example usage with default keypair location. 

```bash
solana create-vote-account $HOME/keypairs/vote-account.json $HOME/keypairs/id.json $(solana address -k $HOME/keypairs/withdraw-account.json) --commission <0-100>
```

If the commission needs to be updated:

```bash
solana vote-update-commission $(solana address -k $HOME/keypairs/vote-account.json) <0-100> $HOME/keypairs/withdraw-account.json
```

## Starting you validator node

IMPORTANT NOTE: You can modify the command to fit your requirements
but the list of --known-validator(s) must be provided.

Save the following commands to a file, such as `~/start-powr.sh` and apply execution permissions with:
```bash
sudo chmod +x ~/start-powr.sh
```

You should change PATH_TO/solana-validator with the location of your solana binary e.g. `/home/validator-admin/.local/share/powr/bin/solana-validator`
You will also need to set your ledger folder to a directory with appropriate storage (recommended 4tb SSD or greater), and the accounts directory, ideally on a separate disk also (recommended 1tb SSD)

You must also change the path of the log file in the `--log` parameter on the last line. A suggested location could be inside the home directory, e.g. `/home/validator-admin/validator-log`

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
--identity /home/validator-admin/keypairs/id.json \
--authorized-voter /home/validator-admin/keypairs/id.json \
--vote-account /home/validator-admin/keypairs/vote-account.json \
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
--identity /home/validator-admin/keypairs/id.json \
--authorized-voter /home/validator-admin/keypairs/id.json \
--vote-account /home/validator-admin/keypairs/vote-account.json \
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

We HIGHLY recommend to manage the process using linux's `systemctl` by issuing the following command


```bash
sudo su
cat > /etc/systemd/system/powr.service << 'EOF'
[Unit]
Description=Powerledger Blockchain Validator Service
Requires=network-online.target
After=network.target

[Service]
LimitNOFILE=1000000
Type=simple
User=validator-admin
WorkingDirectory=/home/validator-admin
ExecStart=/home/validator-admin/start-powr.sh
Restart=always
RestartSec=1

[Install]
WantedBy=multi-user.target
EOF
exit
```

You will then need to enable and start this service, and then check its status
```bash
sudo systemctl enable powr
sudo systemctl start powr
sudo systemctl status powr
```

## System tunning
It is recommended and in some cases required to adjust some system parameters for optimial performance

```bash
# Increase UDP buffers
sudo bash -c "cat >/etc/sysctl.d/20-solana-udp-buffers.conf << 'EOF'
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
sudo bash -c "cat >/etc/sysctl.d/20-solana-mmaps.conf << 'EOF'
# Increase memory mapped files limit
vm.max_map_count = 1000000
EOF"
sudo sysctl -p /etc/sysctl.d/20-solana-mmaps.conf
```

```bash
# Increasing max opened files limit
sudo bash -c "cat >/etc/security/limits.d/90-solana-nofiles.conf << 'EOF'
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

Copy the following commands to create a log rotation. Pleaes enter the location of the log selected earlier.

```bash
sudo su
cat > /etc/logrotate.d/powr.conf << 'EOF'
/PATH_TO_LOGFILE/validator.log {
    rotate 7
    daily
    missingok
    postrotate
        systemctl kill -s USR1 powr.service
    endscript
}
EOF
systemctl restart logrotate.service
exit
```

## Withdrawing tokens from vote-account

```bash
solana withdraw-from-vote-account \
$(solana-keygen pubkey /home/validator-admin/keypairs/vote-account.json) \
<RECIPIENT ADDRESS> \
<AMOUNT> \
--withdraw-authority /home/validator-admin/keypairs/withdraw-account.json
```

## Publish additional information about the validator
You can use the following command to publish a name and URL of your validator. The name will appear on the Powerledger dashboard, explorer and staking website.

```bash
solana validator-info publish -k /home/validator-admin/keypairs/id.json -w "<URL OF THE VALIDATOR>" "<VALIDATOR NAME>"
```

## Getting published validators metadata
You can check the current saved validator information using the following command.

```bash
solana validator-info get
```
