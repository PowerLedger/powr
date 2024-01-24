use solana_sdk::{
    account::{Account, AccountSharedData},
    bpf_loader_upgradeable::UpgradeableLoaderState,
    pubkey::Pubkey,
    rent::Rent,
};

mod spl_token {
    solana_sdk::declare_id!("Token1ZAxcjfmf3ANqs2HEiWXYWHUbkhGynugUn4Joo");
}
mod spl_token_2022 {
    solana_sdk::declare_id!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
}
mod spl_memo_1_0 {
    solana_sdk::declare_id!("MemobGYbxnEG8DZ26obYMiqk7GC2Kda5nGoaaUBYtdL");
}
mod spl_memo_3_0 {
    solana_sdk::declare_id!("MemobGYbxnEG8DZ26obYMiqk7GC2Kda5nGoaaUBYtdL");
}
mod spl_associated_token_account {
    solana_sdk::declare_id!("ATokenqZm7F6ZkbeSGZss84Vrk2nEXacBQGHscAsjq5c");
}

static SPL_PROGRAMS: &[(Pubkey, Pubkey, &[u8])] = &[
    (
        spl_token::ID,
        solana_sdk::bpf_loader::ID,
        include_bytes!("programs/spl_token-3.5.0.so"),
    ),
    (
        spl_token_2022::ID,
        solana_sdk::bpf_loader_upgradeable::ID,
        include_bytes!("programs/spl_token_2022-0.6.0.so"),
    ),
    (
        spl_memo_1_0::ID,
        solana_sdk::bpf_loader::ID,
        include_bytes!("programs/spl_memo-1.0.0.so"),
    ),
    (
        spl_memo_3_0::ID,
        solana_sdk::bpf_loader::ID,
        include_bytes!("programs/spl_memo-3.0.0.so"),
    ),
    (
        spl_associated_token_account::ID,
        solana_sdk::bpf_loader::ID,
        include_bytes!("programs/spl_associated_token_account-1.1.1.so"),
    ),
];

pub fn spl_programs(rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    SPL_PROGRAMS
        .iter()
        .flat_map(|(program_id, loader_id, elf)| {
            let mut accounts = vec![];
            let data = if *loader_id == solana_sdk::bpf_loader_upgradeable::ID {
                let (programdata_address, _) =
                    Pubkey::find_program_address(&[program_id.as_ref()], loader_id);
                let mut program_data = bincode::serialize(&UpgradeableLoaderState::ProgramData {
                    slot: 0,
                    upgrade_authority_address: Some(Pubkey::default()),
                })
                .unwrap();
                program_data.extend_from_slice(elf);
                accounts.push((
                    programdata_address,
                    AccountSharedData::from(Account {
                        lamports: rent.minimum_balance(program_data.len()).max(1),
                        data: program_data,
                        owner: *loader_id,
                        executable: false,
                        rent_epoch: 0,
                    }),
                ));
                bincode::serialize(&UpgradeableLoaderState::Program {
                    programdata_address,
                })
                .unwrap()
            } else {
                elf.to_vec()
            };
            accounts.push((
                *program_id,
                AccountSharedData::from(Account {
                    lamports: rent.minimum_balance(data.len()).max(1),
                    data,
                    owner: *loader_id,
                    executable: true,
                    rent_epoch: 0,
                }),
            ));
            accounts.into_iter()
        })
        .collect()
}
