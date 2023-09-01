//! Collection of all runtime features.
//!
//! Steps to add a new feature are outlined below. Note that these steps only cover
//! the process of getting a feature into the core Solana code.
//! - For features that are unambiguously good (ie bug fixes), these steps are sufficient.
//! - For features that should go up for community vote (ie fee structure changes), more
//!   information on the additional steps to follow can be found at:
//!   <https://spl.solana.com/feature-proposal#feature-proposal-life-cycle>
//!
//! 1. Generate a new keypair with `solana-keygen new --outfile feature.json --no-passphrase`
//!    - Keypairs should be held by core contributors only. If you're a non-core contirbutor going
//!      through these steps, the PR process will facilitate a keypair holder being picked. That
//!      person will generate the keypair, provide pubkey for PR, and ultimately enable the feature.
//! 2. Add a public module for the feature, specifying keypair pubkey as the id with
//!    `solana_sdk::declare_id!()` within the module.
//!    Additionally, add an entry to `FEATURE_NAMES` map.
//! 3. Add desired logic to check for and switch on feature availability.
//!
//! For more information on how features are picked up, see comments for `Feature`.

use {
    lazy_static::lazy_static,
    solana_sdk::{
        clock::Slot,
        hash::{Hash, Hasher},
        pubkey::Pubkey,
    },
    std::collections::{HashMap, HashSet},
};

pub mod deprecate_rewards_sysvar {
    solana_sdk::declare_id!("3uAa9f38v9FiZMUJsjzhQo2c6KwDTWjmapq48i1aEFks");
}

pub mod pico_inflation {
    solana_sdk::declare_id!("4Cf3w5fUMmtP8YCXDLWBo8ZFq6AfD7Six4VSXMbm2MQ5");
}

pub mod full_inflation {
    pub mod devnet_and_testnet {
        solana_sdk::declare_id!("3hZRyARNXejVPCKUD9vb3pmodH9qbpxB5yp8NcLF2E2V");
    }

    pub mod mainnet {
        pub mod certusone {
            pub mod vote {
                solana_sdk::declare_id!("8VbDZLTR4xiLAeyehNE5uMAhgb4gcFGixo8bR6sae6BT");
            }
            pub mod enable {
                solana_sdk::declare_id!("JDhgyVSo16myRutmtJNeEU81BQVLCTm7fxqiwsP1FQYV");
            }
        }
    }
}

pub mod secp256k1_program_enabled {
    solana_sdk::declare_id!("GBwW1RLT5GtfQmXkTW7j5VEg6j78uQEiQDuSpemhMXJ9");
}

pub mod spl_token_v2_multisig_fix {
    solana_sdk::declare_id!("DYWjNcdrHnvMXT6hTRnSjEyvQrFSByxHGSW8gyKBai5Q");
}

pub mod no_overflow_rent_distribution {
    solana_sdk::declare_id!("DKih7jHqVdHEdb3eKfaRUzF86PpfU9zJXDNBSYDQbKCm");
}

pub mod filter_stake_delegation_accounts {
    solana_sdk::declare_id!("As7Whkgu86cwruZZGomkNTEgMBPyycBjXWMwGtLNhp6r");
}

pub mod require_custodian_for_locked_stake_authorize {
    solana_sdk::declare_id!("72Gc6kxSZUor2zGmJiFkHvFf87jFheMpxewVfyfT3DVD");
}

pub mod spl_token_v2_self_transfer_fix {
    solana_sdk::declare_id!("2fgxWTJB6TCZt3hNjQuGtswcYkuQ9e4uos5mTkmH2rBK");
}

pub mod warp_timestamp_again {
    solana_sdk::declare_id!("DApevu7b7Nx21E4WxCM4zy39vfmRCxhSEm2vtbtZSE7F");
}

pub mod check_init_vote_data {
    solana_sdk::declare_id!("8KzeW6oZnehG9fXT7nRqeFFaawXkdxofbzLYRqGrYoW8");
}

pub mod secp256k1_recover_syscall_enabled {
    solana_sdk::declare_id!("9wvo98o8CD13fM5QtXuMMH2a8tRjTYB9KeHrJKieJMJA");
}

pub mod system_transfer_zero_check {
    solana_sdk::declare_id!("EvbUXUzD7MX6Q73WGDYN4YxArpp6uwmVYKnhRGCfzuV9");
}

pub mod blake3_syscall_enabled {
    solana_sdk::declare_id!("G7M2gVMgq87Ut8cEG6w1kq3qBqSN8mqq8t8kro2v6P6a");
}

pub mod dedupe_config_program_signers {
    solana_sdk::declare_id!("C31f3yFzaug8R1jiiVFc2EdbpPnQrcFy7m1oitnmZ35a");
}

pub mod verify_tx_signatures_len {
    solana_sdk::declare_id!("865TgjLehTaFTmTELM419yGGAmQBcdUMDHAjJFEsSjUR");
}

pub mod vote_stake_checked_instructions {
    solana_sdk::declare_id!("H9atLv8WkDaDxL9tLbdMcRRKDZ8UFVoE2jY2aP7qYXhL");
}

pub mod neon_evm_compute_budget {
    solana_sdk::declare_id!("GzTjpaeb2NZsPrMU8oTQ253D7uVe61mMQLV6gzmKxm6D");
}

pub mod rent_for_sysvars {
    solana_sdk::declare_id!("B6KtfnQDb5Sgwvwa4sxDsk2zcTW74V3AMEe6QDpTErVr");
}

pub mod libsecp256k1_0_5_upgrade_enabled {
    solana_sdk::declare_id!("Aa2B5JZ6yGPiMTFSCmQe4PMFuTgdeanSSt1fMFFd2W2a");
}

pub mod tx_wide_compute_cap {
    solana_sdk::declare_id!("74a7bVM8K8Ctkv25dkxtePjKb3cbwpr9CUYTdbYabCez");
}

pub mod spl_token_v2_set_authority_fix {
    solana_sdk::declare_id!("8m5jz71udg4UJL575Cpws5bjG1Jy469nGscPFYmGQZZp");
}

pub mod merge_nonce_error_into_system_error {
    solana_sdk::declare_id!("8PTp5MgXdG3EKi3EdWe3vSNMgcHaEFVuEPUBnDCckWYu");
}

pub mod disable_fees_sysvar {
    solana_sdk::declare_id!("69PAr7umoGk6begvThiSCWz7QcGDYyqR37HArsc377Dv");
}

pub mod stake_merge_with_unmatched_credits_observed {
    solana_sdk::declare_id!("CyDHm4Zx2gm3PdNcJrGcm32dq5hQiHt3bnJnJ5fwph2B");
}

pub mod gate_large_block {
    solana_sdk::declare_id!("4idbuaHGiPgLCrk31LVcSxcnGPCcLCKi24JhH6YthVr4");
}

pub mod zk_token_sdk_enabled {
    solana_sdk::declare_id!("7eb9iUc9ucSL3FpSLhazwvbmPi6ssS8wXY6pN2qP7cRx");
}

pub mod curve25519_syscall_enabled {
    solana_sdk::declare_id!("3WtYmCnQbqjAmp9cW6L1ueTUHx6Pc3mm5ww4dp7AdBpf");
}

pub mod versioned_tx_message_enabled {
    solana_sdk::declare_id!("3zyVNyixPfET4yYjXK5v199PujHDi6HaWXxRuHX5MByG");
}

pub mod libsecp256k1_fail_on_bad_count {
    solana_sdk::declare_id!("CaEcn9nk1XUECg1d4drfYtYRGmhvaApsfBPqUgzmmrcE");
}

pub mod libsecp256k1_fail_on_bad_count2 {
    solana_sdk::declare_id!("JCXbaMyqWus8FqSRM1NnAvjcYmk3qSUKvQk4HhfYf4kQ");
}

pub mod instructions_sysvar_owned_by_sysvar {
    solana_sdk::declare_id!("VPiFsryrrftPgyez5seGmqdewPVvoLYrCRp7ZsqDrjh");
}

pub mod stake_program_advance_activating_credits_observed {
    solana_sdk::declare_id!("EKXsBwSZ98KidxVWcynyB5qFvpKUXTz5RnrJy2uTA6J3");
}

pub mod demote_program_write_locks {
    solana_sdk::declare_id!("5SZhadUfkeewgXgbXdoZ1i61PSYieghiPDWpxCK2SiTh");
}

pub mod ed25519_program_enabled {
    solana_sdk::declare_id!("HFvC53KhBphnXuLje5tedNK2vkVwW2oNi5fgg7NMjGjX");
}

pub mod return_data_syscall_enabled {
    solana_sdk::declare_id!("EQgvQuqSg1N8FKe65sa8K5FU9btHoJwWmkMggoB1VUsq");
}

pub mod reduce_required_deploy_balance {
    solana_sdk::declare_id!("7YiybtRDGg1mfT1agf4v2tyWhtASt4pydo7BVVihbj1G");
}

pub mod sol_log_data_syscall_enabled {
    solana_sdk::declare_id!("8BXPXsu8aR2qy1j9KVyqTnrSeNZ2rXH1xNo9CqC7xHf2");
}

pub mod stakes_remove_delegation_if_inactive {
    solana_sdk::declare_id!("CvXQ8h52XUZ5ffhb3ibvkWASANmaQ6vyHwjN7Nau4vo8");
}

pub mod do_support_realloc {
    solana_sdk::declare_id!("DsNuqVb5ydNydwqfhhcY6pHovKpzgvpxjn7SDtfYcv8a");
}

// Note: when this feature is cleaned up, also remove the secp256k1 program from
// the list of builtins and remove its files from /programs
pub mod prevent_calling_precompiles_as_programs {
    solana_sdk::declare_id!("9g1qLqyMRnTbNDxFUcZLRRsU9xmSc1BFR6n4c51WTktn");
}

pub mod optimize_epoch_boundary_updates {
    solana_sdk::declare_id!("GCBWW6a4Vvw62XoTeNtbBXQznzCGBSqgJyJ3kBJVNL5M");
}

pub mod remove_native_loader {
    solana_sdk::declare_id!("jC1preic9AFZzcwR5dgLV9iqqs3QFrhAeruhwgCauaM");
}

pub mod send_to_tpu_vote_port {
    solana_sdk::declare_id!("EZPfp9D9zxnGjKcFPNsCXm4TxyJbTnvbGMUpfuYyV46P");
}

pub mod requestable_heap_size {
    solana_sdk::declare_id!("b3kDLDVMwPopWzDbhzUechnzUQV3f2njYPt7zkRYQZE");
}

pub mod disable_fee_calculator {
    solana_sdk::declare_id!("E7KC7BaKFithxxWATjZzws5hrAwnzZZXQ3bGQk6Pcu7N");
}

pub mod add_compute_budget_program {
    solana_sdk::declare_id!("CS6eZYvUVvrG126HKxHE3nLhedCWVNuFG7CfeEco3cKG");
}

pub mod nonce_must_be_writable {
    solana_sdk::declare_id!("4HUcQrRTY7e8Mpwsz8aqr1z2Bn83PFizMg8f8bsGoz55");
}

pub mod spl_token_v3_3_0_release {
    solana_sdk::declare_id!("8V6ZGXb4miTqQL8Siys7XtYhwgmDYoYB1QkKznXBwnc5");
}

pub mod leave_nonce_on_success {
    solana_sdk::declare_id!("4UMk7zcqcznMFdYGvg9jjSjudzcUQbzgyCwNg1x2mZR1");
}

pub mod reject_empty_instruction_without_program {
    solana_sdk::declare_id!("7aZj2nhTL42dc1p9kZMXwtfSE2LWooDiiPbQG394KBfu");
}

pub mod fixed_memcpy_nonoverlapping_check {
    solana_sdk::declare_id!("8Ec8TViyhAASK1SjHhYTe1hN67pHb7N9Vbp7prBaU6X9");
}

pub mod reject_non_rent_exempt_vote_withdraws {
    solana_sdk::declare_id!("CjsoFibcUFGcnn87uZqn4TPALGi5WYss5NUZ5wXcEEt7");
}

pub mod evict_invalid_stakes_cache_entries {
    solana_sdk::declare_id!("zX8A8WWEK3EadDoyEDAACSfLHi9DexYPvaTnjdxgD5o");
}

pub mod allow_votes_to_directly_update_vote_state {
    solana_sdk::declare_id!("Br4v723DPmfzAekEZGir4NUv8BjmamBAzWfjqoBA7QLi");
}

pub mod cap_accounts_data_len {
    solana_sdk::declare_id!("GShEFPrTrXmNsMKrLThyMojzEvuJKv4bLotcV3VhGqsc");
}

pub mod max_tx_account_locks {
    solana_sdk::declare_id!("D1fqf88oj4ZSKdTrToFXqB3eTQgcJ172nRYkwvyEy7on");
}

pub mod require_rent_exempt_accounts {
    solana_sdk::declare_id!("SZ8qynruX5MMTWKCUvqgzWttcRs1jxwsobzpTnPXTCM");
}

pub mod filter_votes_outside_slot_hashes {
    solana_sdk::declare_id!("GsHQ135bs8vtuJMsb8eDusRhSG1KC94TrSDdjMMLjefY");
}

pub mod update_syscall_base_costs {
    solana_sdk::declare_id!("2dAGPMrm1HGnj1yqqntctia9mUqiS2nnGxV6G2aZkuNL");
}

pub mod vote_withdraw_authority_may_change_authorized_voter {
    solana_sdk::declare_id!("3LHtLE8uUpDcXyCZ6qxrF7ZuuHfcF7QeZ3NhncmUZE9X");
}

pub mod spl_associated_token_account_v1_0_4 {
    solana_sdk::declare_id!("DPfjV4EwtHSr1aLjTQhwBX7Nu2hognk7RSmBrxFZVNnZ");
}

pub mod reject_vote_account_close_unless_zero_credit_epoch {
    solana_sdk::declare_id!("5FStb9QvJYz4qqV5b2JAK9Y9ysrKxnTZh3NchdQ4YJ6u");
}

pub mod add_get_processed_sibling_instruction_syscall {
    solana_sdk::declare_id!("9z7vh9KwvGGb1XY6MnaXfybHkCFVoMbwwZT7Jgp2izEf");
}

pub mod bank_tranaction_count_fix {
    solana_sdk::declare_id!("AVojnLd1nBZQMQGrV4xVwpqxrq36MeSLwVz8PEgDRHK1");
}

pub mod disable_bpf_deprecated_load_instructions {
    solana_sdk::declare_id!("85USHSF1monSuYFB1xF5UjHtZX3r139nZ5NTNC6APoAs");
}

pub mod disable_bpf_unresolved_symbols_at_runtime {
    solana_sdk::declare_id!("7Dok7goGzXLXLLtWa3mTBb8KbhjCdDWBn53NX894AK1E");
}

pub mod record_instruction_in_transaction_context_push {
    solana_sdk::declare_id!("EKQWunjt1Z1BzbZTSa4i9Hf7dAhDrJNpQoubL7WgofFa");
}

pub mod syscall_saturated_math {
    solana_sdk::declare_id!("5S8DMkPyjtc9T1wiohnCSMEy85UgepyUyJKnBu1n7gaK");
}

pub mod check_physical_overlapping {
    solana_sdk::declare_id!("8YdjovuxeCqrC3PZadLdrBWvfsdpzn5Xmtj8kGqw6YwP");
}

pub mod limit_secp256k1_recovery_id {
    solana_sdk::declare_id!("3vHMMJPBV4BnygyEiouRPgLMfU7E1rWHFpr4wdW1GcoT");
}

pub mod disable_deprecated_loader {
    solana_sdk::declare_id!("42NxW6Nx1DT9SpkdYvDU5u1yEDz3Sjez854FRcWEytYQ");
}

pub mod drop_redundant_turbine_path {
    solana_sdk::declare_id!("DRszsP44r1R33FPXu7auJkZq8tTFnCkTyUZVWFWFGWuP");
}

pub mod spl_token_v3_4_0 {
    solana_sdk::declare_id!("5NQMrNtF2soMkCTNgyvjnkJdGRZnhFAP3w1qebxD1hqK");
}

pub mod spl_associated_token_account_v1_1_0 {
    solana_sdk::declare_id!("BdjScVagw7suwsFD5bV9LjELoJuNshFsaSsR3JYhRwEr");
}

pub mod default_units_per_instruction {
    solana_sdk::declare_id!("F1J33vmZxYGkYWx4x4uP4c8iwi6NmvRMzcsZDLp9iqJr");
}

pub mod stake_allow_zero_undelegated_amount {
    solana_sdk::declare_id!("3rY8xZeFEBqXc4ywvRs8PdktUmutTuCE6Vq7sAuJFsVY");
}

pub mod require_static_program_ids_in_transaction {
    solana_sdk::declare_id!("9VvunPGo6ZnfRZmvWsbdqckpvkEdQpfn2SULBk66iGPq");
}

pub mod add_set_compute_unit_price_ix {
    solana_sdk::declare_id!("AWL5agYvRuRz1LzmPtHAVsgubXoemBaRD5MZhQSDfhp");
}

pub mod include_account_index_in_rent_error {
    solana_sdk::declare_id!("H2aPxEmeVc1BeAdGK7PVWqLQ5mtD411vMU3BJ24BHZTP");
}

pub mod add_shred_type_to_shred_seed {
    solana_sdk::declare_id!("2DC78HxYPgWSp3BpfSH4Ynga8jCJV7AKiCnGYi6H7Dw7");
}

pub mod warp_timestamp_with_a_vengeance {
    solana_sdk::declare_id!("Bzr8XrmwKQpQxoyA465MmBLYcvdcvwhsFbevjP46b9ZZ");
}

pub mod separate_nonce_from_blockhash {
    solana_sdk::declare_id!("3QvxiRvP5Xgp3bvxk71X6ToVJHuyWD2QCCLov6fsk9b2");
}

pub mod enable_durable_nonce {
    solana_sdk::declare_id!("9nZcxdGV63q54txkYmo92XZQtMfAXJHtqzWe8mnpHPEQ");
}

pub mod executables_incur_cpi_data_cost {
    solana_sdk::declare_id!("31DEjA5BFEAb2LouzWFE1KbBU5vpPZiRuLjsX9CDKmEC");
}

pub mod quick_bail_on_panic {
    solana_sdk::declare_id!("HubzYrniAy3Ytqzs8YbRzZBtEGAWq9qFh1Xw7bDEvncW");
}

pub mod nonce_must_be_authorized {
    solana_sdk::declare_id!("CxNAFD7NdS3ZahKWGRzZ2v5mm54umv3Jt9gGaM3himUU");
}

pub mod nonce_must_be_advanceable {
    solana_sdk::declare_id!("AUuSKDGUQZCp2hsUY7aMto8TrmuMxu2nHc1tRhap7GqB");
}

pub mod vote_authorize_with_seed {
    solana_sdk::declare_id!("8NmAj7Rf2FYrRyhm25cNDk7EjqWEnfgcivrNG2ngAPa4");
}

pub mod cap_accounts_data_size_per_block {
    solana_sdk::declare_id!("7PhB1bnKsF8MGNMrCUd4ZGbZpnfaJJ4AmHUPVVDCKveY");
}

pub mod preserve_rent_epoch_for_rent_exempt_accounts {
    solana_sdk::declare_id!("FQWn9mXQWhJHJZUTWYosUnjA1VT6CtJndKHzypmVpAKd");
}

pub mod prevent_crediting_accounts_that_end_rent_paying {
    solana_sdk::declare_id!("7oyoj6fPgxiNA6PK5E3o5SEgsoPPqRn8D9b1HQPP1WP");
}

pub mod return_none_for_zero_lamport_accounts {
    solana_sdk::declare_id!("DzdC5Qzj8kDxt48wsKsa3BEbYS1Gf7etXu2G9hHsMb4j");
}

pub mod increase_tx_account_lock_limit {
    solana_sdk::declare_id!("rteJxyYUJZQRR8G6NXfu1myWm217cyE2qUzhwT6XBVQ");
}

pub mod check_syscall_outputs_do_not_overlap {
    solana_sdk::declare_id!("9dsPsZgBBbbkDPTf2e3DpE9rFBQvA8wZag45PY6hNNcx");
}

lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [
        (secp256k1_program_enabled::id(), "secp256k1 program"),
        (deprecate_rewards_sysvar::id(), "deprecate unused rewards sysvar"),
        (pico_inflation::id(), "pico inflation"),
        (full_inflation::devnet_and_testnet::id(), "full inflation on devnet and testnet"),
        (spl_token_v2_multisig_fix::id(), "spl-token multisig fix"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (filter_stake_delegation_accounts::id(), "filter stake_delegation_accounts #14062"),
        (require_custodian_for_locked_stake_authorize::id(), "require custodian to authorize withdrawer change for locked stake"),
        (spl_token_v2_self_transfer_fix::id(), "spl-token self-transfer fix"),
        (full_inflation::mainnet::certusone::enable::id(), "full inflation enabled by Certus One"),
        (full_inflation::mainnet::certusone::vote::id(), "community vote allowing Certus One to enable full inflation"),
        (warp_timestamp_again::id(), "warp timestamp again, adjust bounding to 25% fast 80% slow #15204"),
        (check_init_vote_data::id(), "check initialized Vote data"),
        (secp256k1_recover_syscall_enabled::id(), "secp256k1_recover syscall"),
        (system_transfer_zero_check::id(), "perform all checks for transfers of 0 lamports"),
        (blake3_syscall_enabled::id(), "blake3 syscall"),
        (dedupe_config_program_signers::id(), "dedupe config program signers"),
        (verify_tx_signatures_len::id(), "prohibit extra transaction signatures"),
        (vote_stake_checked_instructions::id(), "vote/state program checked instructions #18345"),
        (neon_evm_compute_budget::id(), "bump neon_evm's compute budget"),
        (rent_for_sysvars::id(), "collect rent from accounts owned by sysvars"),
        (libsecp256k1_0_5_upgrade_enabled::id(), "upgrade libsecp256k1 to v0.5.0"),
        (tx_wide_compute_cap::id(), "transaction wide compute cap"),
        (spl_token_v2_set_authority_fix::id(), "spl-token set_authority fix"),
        (merge_nonce_error_into_system_error::id(), "merge NonceError into SystemError"),
        (disable_fees_sysvar::id(), "disable fees sysvar"),
        (stake_merge_with_unmatched_credits_observed::id(), "allow merging active stakes with unmatched credits_observed #18985"),
        (gate_large_block::id(), "validator checks block cost against max limit in realtime, reject if exceeds."),
        (zk_token_sdk_enabled::id(), "enable Zk Token proof program and syscalls"),
        (curve25519_syscall_enabled::id(), "enable curve25519 syscalls"),
        (versioned_tx_message_enabled::id(), "enable versioned transaction message processing"),
        (libsecp256k1_fail_on_bad_count::id(), "fail libsec256k1_verify if count appears wrong"),
        (libsecp256k1_fail_on_bad_count2::id(), "fail libsec256k1_verify if count appears wrong"),
        (instructions_sysvar_owned_by_sysvar::id(), "fix owner for instructions sysvar"),
        (stake_program_advance_activating_credits_observed::id(), "Enable advancing credits observed for activation epoch #19309"),
        (demote_program_write_locks::id(), "demote program write locks to readonly, except when upgradeable loader present #19593 #20265"),
        (ed25519_program_enabled::id(), "enable builtin ed25519 signature verify program"),
        (return_data_syscall_enabled::id(), "enable sol_{set,get}_return_data syscall"),
        (reduce_required_deploy_balance::id(), "reduce required payer balance for program deploys"),
        (sol_log_data_syscall_enabled::id(), "enable sol_log_data syscall"),
        (stakes_remove_delegation_if_inactive::id(), "remove delegations from stakes cache when inactive"),
        (do_support_realloc::id(), "support account data reallocation"),
        (prevent_calling_precompiles_as_programs::id(), "prevent calling precompiles as programs"),
        (optimize_epoch_boundary_updates::id(), "optimize epoch boundary updates"),
        (remove_native_loader::id(), "remove support for the native loader"),
        (send_to_tpu_vote_port::id(), "send votes to the tpu vote port"),
        (requestable_heap_size::id(), "Requestable heap frame size"),
        (disable_fee_calculator::id(), "deprecate fee calculator"),
        (add_compute_budget_program::id(), "Add compute_budget_program"),
        (nonce_must_be_writable::id(), "nonce must be writable"),
        (spl_token_v3_3_0_release::id(), "spl-token v3.3.0 release"),
        (leave_nonce_on_success::id(), "leave nonce as is on success"),
        (reject_empty_instruction_without_program::id(), "fail instructions which have native_loader as program_id directly"),
        (fixed_memcpy_nonoverlapping_check::id(), "use correct check for nonoverlapping regions in memcpy syscall"),
        (reject_non_rent_exempt_vote_withdraws::id(), "fail vote withdraw instructions which leave the account non-rent-exempt"),
        (evict_invalid_stakes_cache_entries::id(), "evict invalid stakes cache entries on epoch boundaries"),
        (allow_votes_to_directly_update_vote_state::id(), "enable direct vote state update"),
        (cap_accounts_data_len::id(), "cap the accounts data len"),
        (max_tx_account_locks::id(), "enforce max number of locked accounts per transaction"),
        (require_rent_exempt_accounts::id(), "require all new transaction accounts with data to be rent-exempt"),
        (filter_votes_outside_slot_hashes::id(), "filter vote slots older than the slot hashes history"),
        (update_syscall_base_costs::id(), "update syscall base costs"),
        (vote_withdraw_authority_may_change_authorized_voter::id(), "vote account withdraw authority may change the authorized voter #22521"),
        (spl_associated_token_account_v1_0_4::id(), "SPL Associated Token Account Program release version 1.0.4, tied to token 3.3.0 #22648"),
        (reject_vote_account_close_unless_zero_credit_epoch::id(), "fail vote account withdraw to 0 unless account earned 0 credits in last completed epoch"),
        (add_get_processed_sibling_instruction_syscall::id(), "add add_get_processed_sibling_instruction_syscall"),
        (bank_tranaction_count_fix::id(), "fixes Bank::transaction_count to include all committed transactions, not just successful ones"),
        (disable_bpf_deprecated_load_instructions::id(), "disable ldabs* and ldind* BPF instructions"),
        (disable_bpf_unresolved_symbols_at_runtime::id(), "disable reporting of unresolved BPF symbols at runtime"),
        (record_instruction_in_transaction_context_push::id(), "move the CPI stack overflow check to the end of push"),
        (syscall_saturated_math::id(), "syscalls use saturated math"),
        (check_physical_overlapping::id(), "check physical overlapping regions"),
        (limit_secp256k1_recovery_id::id(), "limit secp256k1 recovery id"),
        (disable_deprecated_loader::id(), "disable the deprecated BPF loader"),
        (drop_redundant_turbine_path::id(), "drop redundant turbine path"),
        (spl_token_v3_4_0::id(), "SPL Token Program version 3.4.0 release #24740"),
        (spl_associated_token_account_v1_1_0::id(), "SPL Associated Token Account Program version 1.1.0 release #24741"),
        (default_units_per_instruction::id(), "Default max tx-wide compute units calculated per instruction"),
        (stake_allow_zero_undelegated_amount::id(), "Allow zero-lamport undelegated amount for initialized stakes #24670"),
        (require_static_program_ids_in_transaction::id(), "require static program ids in versioned transactions"),
        (add_set_compute_unit_price_ix::id(), "add compute budget ix for setting a compute unit price"),
        (include_account_index_in_rent_error::id(), "include account index in rent tx error #25190"),
        (add_shred_type_to_shred_seed::id(), "add shred-type to shred seed #25556"),
        (warp_timestamp_with_a_vengeance::id(), "warp timestamp again, adjust bounding to 150% slow #25666"),
        (separate_nonce_from_blockhash::id(), "separate durable nonce and blockhash domains #25744"),
        (enable_durable_nonce::id(), "enable durable nonce #25744"),
        (executables_incur_cpi_data_cost::id(), "Executables incure CPI data costs"),
        (quick_bail_on_panic::id(), "quick bail on panic"),
        (nonce_must_be_authorized::id(), "nonce must be authorized"),
        (nonce_must_be_advanceable::id(), "durable nonces must be advanceable"),
        (vote_authorize_with_seed::id(), "An instruction you can use to change a vote accounts authority when the current authority is a derived key #25860"),
        (cap_accounts_data_size_per_block::id(), "cap the accounts data size per block #25517"),
        (preserve_rent_epoch_for_rent_exempt_accounts::id(), "preserve rent epoch for rent exempt accounts #26479"),
        (prevent_crediting_accounts_that_end_rent_paying::id(), "prevent crediting rent paying accounts #26606"),
        (return_none_for_zero_lamport_accounts::id(), "return none for zero lamport accounts #27800"),
        (increase_tx_account_lock_limit::id(), "increase tx account lock limit to 128 #27241"),
        (check_syscall_outputs_do_not_overlap::id(), "check syscall outputs do_not overlap #28600"),
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

    /// Unique identifier of the current software's feature set
    pub static ref ID: Hash = {
        let mut hasher = Hasher::default();
        let mut feature_ids = FEATURE_NAMES.keys().collect::<Vec<_>>();
        feature_ids.sort();
        for feature in feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FullInflationFeaturePair {
    pub vote_id: Pubkey, // Feature that grants the candidate the ability to enable full inflation
    pub enable_id: Pubkey, // Feature to enable full inflation by the candidate
}

lazy_static! {
    /// Set of feature pairs that once enabled will trigger full inflation
    pub static ref FULL_INFLATION_FEATURE_PAIRS: HashSet<FullInflationFeaturePair> = [
        FullInflationFeaturePair {
            vote_id: full_inflation::mainnet::certusone::vote::id(),
            enable_id: full_inflation::mainnet::certusone::enable::id(),
        },
    ]
    .iter()
    .cloned()
    .collect();
}

/// `FeatureSet` holds the set of currently active/inactive runtime features
#[derive(AbiExample, Debug, Clone)]
pub struct FeatureSet {
    pub active: HashMap<Pubkey, Slot>,
    pub inactive: HashSet<Pubkey>,
}
impl Default for FeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            active: HashMap::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
impl FeatureSet {
    pub fn is_active(&self, feature_id: &Pubkey) -> bool {
        self.active.contains_key(feature_id)
    }

    pub fn activated_slot(&self, feature_id: &Pubkey) -> Option<Slot> {
        self.active.get(feature_id).copied()
    }

    /// List of enabled features that trigger full inflation
    pub fn full_inflation_features_enabled(&self) -> HashSet<Pubkey> {
        let mut hash_set = FULL_INFLATION_FEATURE_PAIRS
            .iter()
            .filter_map(|pair| {
                if self.is_active(&pair.vote_id) && self.is_active(&pair.enable_id) {
                    Some(pair.enable_id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        if self.is_active(&full_inflation::devnet_and_testnet::id()) {
            hash_set.insert(full_inflation::devnet_and_testnet::id());
        }
        hash_set
    }

    /// All features enabled, useful for testing
    pub fn all_enabled() -> Self {
        Self {
            active: FEATURE_NAMES.keys().cloned().map(|key| (key, 0)).collect(),
            inactive: HashSet::new(),
        }
    }

    /// Activate a feature
    pub fn activate(&mut self, feature_id: &Pubkey, slot: u64) {
        self.inactive.remove(feature_id);
        self.active.insert(*feature_id, slot);
    }

    /// Deactivate a feature
    pub fn deactivate(&mut self, feature_id: &Pubkey) {
        self.active.remove(feature_id);
        self.inactive.insert(*feature_id);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_inflation_features_enabled_devnet_and_testnet() {
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::devnet_and_testnet::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::devnet_and_testnet::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_full_inflation_features_enabled() {
        // Normal sequence: vote_id then enable_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );

        // Backwards sequence: enable_id and then vote_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_feature_set_activate_deactivate() {
        let mut feature_set = FeatureSet::default();

        let feature = Pubkey::new_unique();
        assert!(!feature_set.is_active(&feature));
        feature_set.activate(&feature, 0);
        assert!(feature_set.is_active(&feature));
        feature_set.deactivate(&feature);
        assert!(!feature_set.is_active(&feature));
    }
}
