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
//!    - Keypairs should be held by core contributors only. If you're a non-core contributor going
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
    solana_program::{epoch_schedule::EpochSchedule, stake_history::Epoch},
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

pub mod credits_auto_rewind {
    solana_sdk::declare_id!("ArhevNNpdiUXFBac2cu7xYbdjfMVsyPmKGSzY2u19qhR");
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

pub mod stake_deactivate_delinquent_instruction {
    solana_sdk::declare_id!("3yP3EGjWD7SwZvk9mALoFn27Uirtoff9XrFjQ1wXLXp3");
}

pub mod stake_redelegate_instruction {
    solana_sdk::declare_id!("3w5rpavyBaRrGYTzsmhQui7LWTSP3rPSLnPBYqNxyNK7");
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

pub mod bank_transaction_count_fix {
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

pub mod check_slice_translation_size {
    solana_sdk::declare_id!("9BEfs9dbwZ3DA7dsGWYiHbedAuu1NGfVTBXSBnVzRiv9");
}

pub mod stake_split_uses_rent_sysvar {
    solana_sdk::declare_id!("8f2zrt1UpyEJpLfY2G1CxrdFbSKhpE2T384RHJLoLU1K");
}

pub mod add_get_minimum_delegation_instruction_to_stake_program {
    solana_sdk::declare_id!("CzhPo9NMSJYRLUe8x9A6XLCnsBn5KuXUX2qBi4jJchbp");
}

pub mod error_on_syscall_bpf_function_hash_collisions {
    solana_sdk::declare_id!("GaJfrub1QDjfVgEy56zKJfDTzawHLq9bF7aMAUb37cYG");
}

pub mod reject_callx_r10 {
    solana_sdk::declare_id!("9QibqkiL4Gmcse92joeBQPZ3kY3pSQ9VhdiDSo6bk848");
}

pub mod drop_redundant_turbine_path {
    solana_sdk::declare_id!("DRszsP44r1R33FPXu7auJkZq8tTFnCkTyUZVWFWFGWuP");
}

pub mod executables_incur_cpi_data_cost {
    solana_sdk::declare_id!("ATjywpeivPjaLfEDVXds5UKTDKpiPnfzeWsiL5YeF5ZL");
}

pub mod fix_recent_blockhashes {
    solana_sdk::declare_id!("36d5FF2huRwCigmXpntMvLw8wuHzgLpejv37KL5CtdAd");
}

pub mod update_rewards_from_cached_accounts {
    solana_sdk::declare_id!("5vmBmqeVabKYz3ftgd9eZhrtvTkj8QMAW47bcZpnvX9L");
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

pub mod stake_raise_minimum_delegation_to_1_sol {
    // This is a feature-proposal *feature id*.  The feature keypair address is `GQXzC7YiSNkje6FFUk6sc2p53XRvKoaZ9VMktYzUMnpL`.
    solana_sdk::declare_id!("FeqLA35GSSrNaSqovy7gPm5k7ufcX2PWhNutjqvARsaf");
}

pub mod stake_minimum_delegation_for_rewards {
    solana_sdk::declare_id!("FJeuDjSo237p27SUCLXJs8CMzUdYn5cK8rcc9nPaF8v5");
}

pub mod add_set_compute_unit_price_ix {
    solana_sdk::declare_id!("AWL5agYvRuRz1LzmPtHAVsgubXoemBaRD5MZhQSDfhp");
}

pub mod disable_deploy_of_alloc_free_syscall {
    solana_sdk::declare_id!("D9AR6yzmWcRdT8RtaYeTMnXbASH9LMG7Yi8CJ9kn4Ehs");
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

pub mod vote_state_update_credit_per_dequeue {
    solana_sdk::declare_id!("9a6QJLpUMCTE3dJVVrQBFfeS4nAaagdFJP1qmTK3Swet");
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

pub mod enable_bpf_loader_extend_program_ix {
    solana_sdk::declare_id!("iQkFmvHLmzYpxjom9aFAkHJ4Je9SFbjefMqCRuP1313");
}

pub mod enable_early_verification_of_account_modifications {
    solana_sdk::declare_id!("5RZmkjDxtZYWhjt9CFfzprNztDapAXCgtuS2zUdHBhoX");
}

pub mod skip_rent_rewrites {
    solana_sdk::declare_id!("5jv2WHdKHwSQfafhmeWHzN8sqPsbvusDeic6Hr4Hh2ef");
}

pub mod prevent_crediting_accounts_that_end_rent_paying {
    solana_sdk::declare_id!("7oyoj6fPgxiNA6PK5E3o5SEgsoPPqRn8D9b1HQPP1WP");
}

pub mod cap_bpf_program_instruction_accounts {
    solana_sdk::declare_id!("8RmWEiW9f1PNGaXa9N22aHFf32M78vpKD1NDqF1xxCQx");
}

pub mod loosen_cpi_size_restriction {
    solana_sdk::declare_id!("EMmECA7Bcd2VrX7eLZvM6779RLuaFWX8FWyjB6hTnZD2");
}

pub mod use_default_units_in_fee_calculation {
    solana_sdk::declare_id!("P7TF7KySzmXS8QH2eHFQmvnadjFogD8z6tQ8SxD9454");
}

pub mod compact_vote_state_updates {
    solana_sdk::declare_id!("6c34D12qwpiXXzwfU2bNfWeSWvsd1RovBeJma4L1NRrT");
}

pub mod incremental_snapshot_only_incremental_hash_calculation {
    solana_sdk::declare_id!("5tsmjyahowwVNpefVUf4A6gQWTzKqGmKupq1NPkTnCmF");
}

pub mod disable_cpi_setting_executable_and_rent_epoch {
    solana_sdk::declare_id!("BNKra2ZKHmVLWh6EGgzUMyRL3FVBdsjartt3sFqRhqnp");
}

pub mod on_load_preserve_rent_epoch_for_rent_exempt_accounts {
    solana_sdk::declare_id!("3YoVxZh7hS7a9YsFYqAFXwTn2RFAcoZ6SheR5S9P5A4i");
}

pub mod account_hash_ignore_slot {
    solana_sdk::declare_id!("4Cx1NeZJAjHd86MZbRqRQcpM1orKFPM7nUnPjZgwK5Hv");
}

pub mod set_exempt_rent_epoch_max {
    solana_sdk::declare_id!("99h4V7kHr4xC1NJhDTfn23VJC4LaNPQQRfuC7pREH35a");
}

pub mod relax_authority_signer_check_for_lookup_table_creation {
    solana_sdk::declare_id!("ECw1TBnAVMFvzhmRmx9w8mrAV2jcrL9dMmrJqovchgoh");
}

pub mod stop_sibling_instruction_search_at_parent {
    solana_sdk::declare_id!("3oDA58JvPXf58v4WmVQjfJWV2fFDdAbE4iLELQfBnhdG");
}

pub mod vote_state_update_root_fix {
    solana_sdk::declare_id!("AjzVZB5K6YkWSwujkL2BrGAk9X2G9CYjX2NRDbhQhctR");
}

pub mod cap_accounts_data_allocations_per_transaction {
    solana_sdk::declare_id!("CDzQaJgvRVMRz7YtBQXyAzaVQxmmV9fPe2TYKWaWGvXB");
}

pub mod epoch_accounts_hash {
    solana_sdk::declare_id!("D2mYpB7BtsaBKn7ZDZqFs1ktkxbLvQSG5GbgnX24W3bZ");
}

pub mod remove_deprecated_request_unit_ix {
    solana_sdk::declare_id!("34UwX4Y614tm8cTLNNUtvbJoseNAYdRyZFjoprithUCt");
}

pub mod disable_rehash_for_rent_epoch {
    solana_sdk::declare_id!("CZomNWPRLVeaLdj3wVPkq643ck2UHUvYAaVe6B5fYMic");
}

pub mod increase_tx_account_lock_limit {
    solana_sdk::declare_id!("rteJxyYUJZQRR8G6NXfu1myWm217cyE2qUzhwT6XBVQ");
}

pub mod limit_max_instruction_trace_length {
    solana_sdk::declare_id!("EEBZp3qQ2d1DAAN9PV8pGhPSv5s21gaoctJkUqRbFsww");
}

pub mod check_syscall_outputs_do_not_overlap {
    solana_sdk::declare_id!("9dsPsZgBBbbkDPTf2e3DpE9rFBQvA8wZag45PY6hNNcx");
}

pub mod enable_bpf_loader_set_authority_checked_ix {
    solana_sdk::declare_id!("6Ly9DFBvVjNVCe6waccTLGztF2zhXuQ71feBhuDKHfUJ");
}

pub mod enable_alt_bn128_syscall {
    solana_sdk::declare_id!("5a1b5kxbp8DXory5zeehiCgGMJjTdN9f4UBK3SwrKHd1");
}

pub mod enable_program_redeployment_cooldown {
    solana_sdk::declare_id!("BL679yU6dtBy5quFYF8h2LjFeBQyQr3X6FUzw4LzTXNj");
}

pub mod commission_updates_only_allowed_in_first_half_of_epoch {
    solana_sdk::declare_id!("cyF3n7F3UsaymynHV1cfaVbYB7f6oNURxDgW63SF3m4");
}

pub mod enable_turbine_fanout_experiments {
    solana_sdk::declare_id!("9nc5DDQ3rPnNpQ7CwwxjMtvPUY3456g9YqrXeAsSPzQ3");
}

pub mod disable_turbine_fanout_experiments {
    solana_sdk::declare_id!("2to6WNVUhLSiFXtUaHE4PqAgFDmHn3J6nYs7GaQDLU91");
}

pub mod drop_merkle_shreds {
    solana_sdk::declare_id!("4kGkPL4SL9UWgaZYfc8kftUXQfwco81NoMu7XAhJ7zju");
}

pub mod keep_merkle_shreds {
    solana_sdk::declare_id!("HPFsFrBcFNWjJyabEYiZdthLxmSzVXh3prpsFtqYLWFb");
}

pub mod move_serialized_len_ptr_in_cpi {
    solana_sdk::declare_id!("FXF9zcswXHtWYXacAgmQSNnK52bKEvTej3URuchQYHhn");
}

pub mod update_hashes_per_tick {
    solana_sdk::declare_id!("AuQesVJGhdagas9a2MMWVCMg8LzzcZmyYvza1hkbHhQk");
}

pub mod enable_big_mod_exp_syscall {
    solana_sdk::declare_id!("5iABppAAB27GnpLEQKV7kJJm5TEzsTmFCgdkeE3hJKxi");
}

pub mod disable_builtin_loader_ownership_chains {
    solana_sdk::declare_id!("5qirP859NrW97Ne51C61wLhc9FHeEgP94p9vvun4eJ4N");
}

pub mod cap_transaction_accounts_data_size {
    solana_sdk::declare_id!("4MbfBfn7xw5wo2BwUUvLLHrhyS9WCggHCG9MQCxkmDXx");
}

pub mod remove_congestion_multiplier_from_fee_calculation {
    solana_sdk::declare_id!("J3C2TqCeKX1x5awvdi5wGAL1Bx6uCyVCer8kQ1eQL74n");
}

pub mod enable_request_heap_frame_ix {
    solana_sdk::declare_id!("BcJ935ni25SsRizyCBiU2JVpF2SRUZUAKsoW5g1buZXG");
}

pub mod prevent_rent_paying_rent_recipients {
    solana_sdk::declare_id!("HSaFnh66UKzW1j7xdsoXVhwyETCgXTAfsPrc9fbvQjuK");
}

pub mod delay_visibility_of_program_deployment {
    solana_sdk::declare_id!("4WWiETd7mh4FbXMxWv6csjoSqygeNu1cG5qqKaRne96v");
}

pub mod apply_cost_tracker_during_replay {
    solana_sdk::declare_id!("5zng9rY3Sw6ttFAMihsTHMvVQ5KpfHR517jwzneyygL7");
}
pub mod bpf_account_data_direct_mapping {
    solana_sdk::declare_id!("55uMCkEH2ypYmh83cqhDiwwQLDtYjHcC7bDHAdjv4BR7");
}

pub mod add_set_tx_loaded_accounts_data_size_instruction {
    solana_sdk::declare_id!("G387PLAvgpxdbujR4ov3jjsjsB8AiZsqnY7xD5K7Kz2");
}

pub mod switch_to_new_elf_parser {
    solana_sdk::declare_id!("5xo1TWXFfWGAUgy4WDRQ4jV5FiLCsRd5wxViy7p3vVnd");
}

pub mod round_up_heap_size {
    solana_sdk::declare_id!("GGpsBFnACAh18uQuRSHWWPE6X6T9zUYKBe5WceZSKmaS");
}

pub mod remove_bpf_loader_incorrect_program_id {
    solana_sdk::declare_id!("4v2LrRpCBcfeLuJaMksDkJTpZJwfQtRcAzpZcP15XfV2");
}

pub mod include_loaded_accounts_data_size_in_fee_calculation {
    solana_sdk::declare_id!("EjuP2JRRLy84Sk9cEqWXS2swgUsuDrBUnbpcVBm6r6HH");
}

pub mod native_programs_consume_cu {
    solana_sdk::declare_id!("G1vcDdsY3Jxe85y45NnxWFDtbtMCucoLzmNFwCbHmkKx");
}

pub mod simplify_writable_program_account_check {
    solana_sdk::declare_id!("2Ch3M4DvKDvCpxm9ReKzq49bMweFTf29opFu31f9wUAf");
}

pub mod stop_truncating_strings_in_syscalls {
    solana_sdk::declare_id!("8qeJVAavJ7qTcbcGq6GDdnKFdAgNjHJgXdLLCnw7BD6G");
}

pub mod clean_up_delegation_errors {
    solana_sdk::declare_id!("GkBgWXFQ6XyuykHNDc7LohGBRDMaXXhERQgTQ7dQ98GD");
}

pub mod vote_state_add_vote_latency {
    solana_sdk::declare_id!("DtpwgQoK5bz5b5V9BEmfFrgnj3HGwUo1fzSFsnkzRrf8");
}

pub mod checked_arithmetic_in_fee_validation {
    solana_sdk::declare_id!("BFbEEEbNMgnfUfnGuCxnKM3EmAKXmr3VfZSUMcRmRzPK");
}

pub mod reduce_stake_warmup_cooldown {
    solana_sdk::declare_id!("D3bUFPo7nAQ159VLsC2zcMZiHWkuhR6tM9nVKDhBhY3i");
}

pub mod revise_turbine_epoch_stakes {
    solana_sdk::declare_id!("9Swq9No8745L9x9XuXHPYkwu2pAzDuAekAwbaj8oNRZC");
}

pub mod require_rent_exempt_split_destination {
    solana_sdk::declare_id!("58iruVddimi2pEnwg3T4LTdfguFbtVMqP1T1HMrCZ3MF");
}

pub mod better_error_codes_for_tx_lamport_check {
    solana_sdk::declare_id!("ERbutGc25FqFTdJ58PUJVuijdndqwTkWjhoFWe2x2dpb");
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
        (rent_for_sysvars::id(), "collect rent from accounts owned by sysvars"),
        (libsecp256k1_0_5_upgrade_enabled::id(), "upgrade libsecp256k1 to v0.5.0"),
        (tx_wide_compute_cap::id(), "transaction wide compute cap"),
        (spl_token_v2_set_authority_fix::id(), "spl-token set_authority fix"),
        (merge_nonce_error_into_system_error::id(), "merge NonceError into SystemError"),
        (disable_fees_sysvar::id(), "disable fees sysvar"),
        (stake_merge_with_unmatched_credits_observed::id(), "allow merging active stakes with unmatched credits_observed #18985"),
        (zk_token_sdk_enabled::id(), "enable Zk Token proof program and syscalls"),
        (curve25519_syscall_enabled::id(), "enable curve25519 syscalls"),
        (versioned_tx_message_enabled::id(), "enable versioned transaction message processing"),
        (libsecp256k1_fail_on_bad_count::id(), "fail libsec256k1_verify if count appears wrong"),
        (libsecp256k1_fail_on_bad_count2::id(), "fail libsec256k1_verify if count appears wrong"),
        (instructions_sysvar_owned_by_sysvar::id(), "fix owner for instructions sysvar"),
        (stake_program_advance_activating_credits_observed::id(), "Enable advancing credits observed for activation epoch #19309"),
        (credits_auto_rewind::id(), "Auto rewind stake's credits_observed if (accidental) vote recreation is detected #22546"),
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
        (stake_deactivate_delinquent_instruction::id(), "enable the deactivate delinquent stake instruction #23932"),
        (vote_withdraw_authority_may_change_authorized_voter::id(), "vote account withdraw authority may change the authorized voter #22521"),
        (spl_associated_token_account_v1_0_4::id(), "SPL Associated Token Account Program release version 1.0.4, tied to token 3.3.0 #22648"),
        (reject_vote_account_close_unless_zero_credit_epoch::id(), "fail vote account withdraw to 0 unless account earned 0 credits in last completed epoch"),
        (add_get_processed_sibling_instruction_syscall::id(), "add add_get_processed_sibling_instruction_syscall"),
        (bank_transaction_count_fix::id(), "fixes Bank::transaction_count to include all committed transactions, not just successful ones"),
        (disable_bpf_deprecated_load_instructions::id(), "disable ldabs* and ldind* SBF instructions"),
        (disable_bpf_unresolved_symbols_at_runtime::id(), "disable reporting of unresolved SBF symbols at runtime"),
        (record_instruction_in_transaction_context_push::id(), "move the CPI stack overflow check to the end of push"),
        (syscall_saturated_math::id(), "syscalls use saturated math"),
        (check_physical_overlapping::id(), "check physical overlapping regions"),
        (limit_secp256k1_recovery_id::id(), "limit secp256k1 recovery id"),
        (disable_deprecated_loader::id(), "disable the deprecated BPF loader"),
        (check_slice_translation_size::id(), "check size when translating slices"),
        (stake_split_uses_rent_sysvar::id(), "stake split instruction uses rent sysvar"),
        (add_get_minimum_delegation_instruction_to_stake_program::id(), "add GetMinimumDelegation instruction to stake program"),
        (error_on_syscall_bpf_function_hash_collisions::id(), "error on bpf function hash collisions"),
        (reject_callx_r10::id(), "Reject bpf callx r10 instructions"),
        (drop_redundant_turbine_path::id(), "drop redundant turbine path"),
        (executables_incur_cpi_data_cost::id(), "Executables incur CPI data costs"),
        (fix_recent_blockhashes::id(), "stop adding hashes for skipped slots to recent blockhashes"),
        (update_rewards_from_cached_accounts::id(), "update rewards from cached accounts"),
        (spl_token_v3_4_0::id(), "SPL Token Program version 3.4.0 release #24740"),
        (spl_associated_token_account_v1_1_0::id(), "SPL Associated Token Account Program version 1.1.0 release #24741"),
        (default_units_per_instruction::id(), "Default max tx-wide compute units calculated per instruction"),
        (stake_allow_zero_undelegated_amount::id(), "Allow zero-lamport undelegated amount for initialized stakes #24670"),
        (require_static_program_ids_in_transaction::id(), "require static program ids in versioned transactions"),
        (stake_raise_minimum_delegation_to_1_sol::id(), "Raise minimum stake delegation to 1.0 SOL #24357"),
        (stake_minimum_delegation_for_rewards::id(), "stakes must be at least the minimum delegation to earn rewards"),
        (add_set_compute_unit_price_ix::id(), "add compute budget ix for setting a compute unit price"),
        (disable_deploy_of_alloc_free_syscall::id(), "disable new deployments of deprecated sol_alloc_free_ syscall"),
        (include_account_index_in_rent_error::id(), "include account index in rent tx error #25190"),
        (add_shred_type_to_shred_seed::id(), "add shred-type to shred seed #25556"),
        (warp_timestamp_with_a_vengeance::id(), "warp timestamp again, adjust bounding to 150% slow #25666"),
        (separate_nonce_from_blockhash::id(), "separate durable nonce and blockhash domains #25744"),
        (enable_durable_nonce::id(), "enable durable nonce #25744"),
        (vote_state_update_credit_per_dequeue::id(), "Calculate vote credits for VoteStateUpdate per vote dequeue to match credit awards for Vote instruction"),
        (quick_bail_on_panic::id(), "quick bail on panic"),
        (nonce_must_be_authorized::id(), "nonce must be authorized"),
        (nonce_must_be_advanceable::id(), "durable nonces must be advanceable"),
        (vote_authorize_with_seed::id(), "An instruction you can use to change a vote accounts authority when the current authority is a derived key #25860"),
        (cap_accounts_data_size_per_block::id(), "cap the accounts data size per block #25517"),
        (stake_redelegate_instruction::id(), "enable the redelegate stake instruction #26294"),
        (preserve_rent_epoch_for_rent_exempt_accounts::id(), "preserve rent epoch for rent exempt accounts #26479"),
        (enable_bpf_loader_extend_program_ix::id(), "enable bpf upgradeable loader ExtendProgram instruction #25234"),
        (skip_rent_rewrites::id(), "skip rewriting rent exempt accounts during rent collection #26491"),
        (enable_early_verification_of_account_modifications::id(), "enable early verification of account modifications #25899"),
        (disable_rehash_for_rent_epoch::id(), "on accounts hash calculation, do not try to rehash accounts #28934"),
        (account_hash_ignore_slot::id(), "ignore slot when calculating an account hash #28420"),
        (set_exempt_rent_epoch_max::id(), "set rent epoch to Epoch::MAX for rent-exempt accounts #28683"),
        (on_load_preserve_rent_epoch_for_rent_exempt_accounts::id(), "on bank load account, do not try to fix up rent_epoch #28541"),
        (prevent_crediting_accounts_that_end_rent_paying::id(), "prevent crediting rent paying accounts #26606"),
        (cap_bpf_program_instruction_accounts::id(), "enforce max number of accounts per bpf program instruction #26628"),
        (loosen_cpi_size_restriction::id(), "loosen cpi size restrictions #26641"),
        (use_default_units_in_fee_calculation::id(), "use default units per instruction in fee calculation #26785"),
        (compact_vote_state_updates::id(), "Compact vote state updates to lower block size"),
        (incremental_snapshot_only_incremental_hash_calculation::id(), "only hash accounts in incremental snapshot during incremental snapshot creation #26799"),
        (disable_cpi_setting_executable_and_rent_epoch::id(), "disable setting is_executable and_rent_epoch in CPI #26987"),
        (relax_authority_signer_check_for_lookup_table_creation::id(), "relax authority signer check for lookup table creation #27205"),
        (stop_sibling_instruction_search_at_parent::id(), "stop the search in get_processed_sibling_instruction when the parent instruction is reached #27289"),
        (vote_state_update_root_fix::id(), "fix root in vote state updates #27361"),
        (cap_accounts_data_allocations_per_transaction::id(), "cap accounts data allocations per transaction #27375"),
        (epoch_accounts_hash::id(), "enable epoch accounts hash calculation #27539"),
        (remove_deprecated_request_unit_ix::id(), "remove support for RequestUnitsDeprecated instruction #27500"),
        (increase_tx_account_lock_limit::id(), "increase tx account lock limit to 128 #27241"),
        (limit_max_instruction_trace_length::id(), "limit max instruction trace length #27939"),
        (check_syscall_outputs_do_not_overlap::id(), "check syscall outputs do_not overlap #28600"),
        (enable_bpf_loader_set_authority_checked_ix::id(), "enable bpf upgradeable loader SetAuthorityChecked instruction #28424"),
        (enable_alt_bn128_syscall::id(), "add alt_bn128 syscalls #27961"),
        (enable_program_redeployment_cooldown::id(), "enable program redeployment cooldown #29135"),
        (commission_updates_only_allowed_in_first_half_of_epoch::id(), "validator commission updates are only allowed in the first half of an epoch #29362"),
        (enable_turbine_fanout_experiments::id(), "enable turbine fanout experiments #29393"),
        (disable_turbine_fanout_experiments::id(), "disable turbine fanout experiments #29393"),
        (drop_merkle_shreds::id(), "drop merkle shreds #29711"),
        (keep_merkle_shreds::id(), "keep merkle shreds #29711"),
        (move_serialized_len_ptr_in_cpi::id(), "cpi ignore serialized_len_ptr #29592"),
        (update_hashes_per_tick::id(), "Update desired hashes per tick on epoch boundary"),
        (enable_big_mod_exp_syscall::id(), "add big_mod_exp syscall #28503"),
        (disable_builtin_loader_ownership_chains::id(), "disable builtin loader ownership chains #29956"),
        (cap_transaction_accounts_data_size::id(), "cap transaction accounts data size up to a limit #27839"),
        (remove_congestion_multiplier_from_fee_calculation::id(), "Remove congestion multiplier from transaction fee calculation #29881"),
        (enable_request_heap_frame_ix::id(), "Enable transaction to request heap frame using compute budget instruction #30076"),
        (prevent_rent_paying_rent_recipients::id(), "prevent recipients of rent rewards from ending in rent-paying state #30151"),
        (delay_visibility_of_program_deployment::id(), "delay visibility of program upgrades #30085"),
        (apply_cost_tracker_during_replay::id(), "apply cost tracker to blocks during replay #29595"),
        (add_set_tx_loaded_accounts_data_size_instruction::id(), "add compute budget instruction for setting account data size per transaction #30366"),
        (switch_to_new_elf_parser::id(), "switch to new ELF parser #30497"),
        (round_up_heap_size::id(), "round up heap size when calculating heap cost #30679"),
        (remove_bpf_loader_incorrect_program_id::id(), "stop incorrectly throwing IncorrectProgramId in bpf_loader #30747"),
        (include_loaded_accounts_data_size_in_fee_calculation::id(), "include transaction loaded accounts data size in base fee calculation #30657"),
        (native_programs_consume_cu::id(), "Native program should consume compute units #30620"),
        (simplify_writable_program_account_check::id(), "Simplify checks performed for writable upgradeable program accounts #30559"),
        (stop_truncating_strings_in_syscalls::id(), "Stop truncating strings in syscalls #31029"),
        (clean_up_delegation_errors::id(), "Return InsufficientDelegation instead of InsufficientFunds or InsufficientStake where applicable #31206"),
        (vote_state_add_vote_latency::id(), "replace Lockout with LandedVote (including vote latency) in vote state #31264"),
        (checked_arithmetic_in_fee_validation::id(), "checked arithmetic in fee validation #31273"),
        (bpf_account_data_direct_mapping::id(), "use memory regions to map account data into the rbpf vm instead of copying the data"),
        (reduce_stake_warmup_cooldown::id(), "reduce stake warmup cooldown from 25% to 9%"),
        (revise_turbine_epoch_stakes::id(), "revise turbine epoch stakes"),
        (require_rent_exempt_split_destination::id(), "Require stake split destination account to be rent exempt"),
        (better_error_codes_for_tx_lamport_check::id(), "better error codes for tx lamport check #33353"),
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
#[derive(AbiExample, Debug, Clone, Eq, PartialEq)]
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

    pub fn new_warmup_cooldown_rate_epoch(&self, epoch_schedule: &EpochSchedule) -> Option<Epoch> {
        self.activated_slot(&reduce_stake_warmup_cooldown::id())
            .map(|slot| epoch_schedule.get_epoch(slot))
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
