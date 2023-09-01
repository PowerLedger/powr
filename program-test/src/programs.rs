use solana_sdk::{
    account::{Account, AccountSharedData},
    pubkey::Pubkey,
    rent::Rent,
};

mod spl_token {
    solana_sdk::declare_id!("Token1ZAxcjfmf3ANqs2HEiWXYWHUbkhGynugUn4Joo");
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

static SPL_PROGRAMS: &[(Pubkey, &[u8])] = &[
    (spl_token::ID, include_bytes!("programs/spl_token-3.5.0.so")),
    (
        spl_memo_1_0::ID,
        include_bytes!("programs/spl_memo-1.0.0.so"),
    ),
    (
        spl_memo_3_0::ID,
        include_bytes!("programs/spl_memo-3.0.0.so"),
    ),
    (
        spl_associated_token_account::ID,
        include_bytes!("programs/spl_associated_token_account-1.1.1.so"),
    ),
];

pub fn spl_programs(rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    SPL_PROGRAMS
        .iter()
        .map(|(program_id, elf)| {
            (
                *program_id,
                AccountSharedData::from(Account {
                    lamports: rent.minimum_balance(elf.len()).min(1),
                    data: elf.to_vec(),
                    owner: solana_sdk::bpf_loader::id(),
                    executable: true,
                    rent_epoch: 0,
                }),
            )
        })
        .collect()
}
