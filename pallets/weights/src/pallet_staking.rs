// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-19, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512
//! HOSTNAME: `ubuntu-8gb-hel1-1`, CPU: `Intel Xeon Processor (Skylake, IBRS)`

// Executed Command:
// ./target/release/polymesh
// benchmark
// pallet
// -s
// 100
// -r
// 5
// -p=pallet_staking
// -e=*
// --heap-pages
// 4096
// --db-cache
// 512
// --execution
// wasm
// --wasm-execution
// compiled
// --output
// ./pallets/weights/src/
// --template
// ./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use polymesh_runtime_common::{RocksDbWeight as DbWeight, Weight};

/// Weights for pallet_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_staking::WeightInfo for SubstrateWeight {
    // Storage: Staking Bonded (r:1 w:1)
    // Storage: Staking Ledger (r:1 w:1)
    // Storage: Staking CurrentEra (r:1 w:0)
    // Storage: Staking HistoryDepth (r:1 w:0)
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: Staking Payee (r:0 w:1)
    fn bond() -> Weight {
        // Minimum execution time: 152_054 nanoseconds.
        Weight::from_ref_time(155_300_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking Bonded (r:1 w:0)
    // Storage: Staking Ledger (r:1 w:1)
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Balances Locks (r:1 w:1)
    fn bond_extra() -> Weight {
        // Minimum execution time: 129_703 nanoseconds.
        Weight::from_ref_time(133_277_000)
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking Ledger (r:1 w:1)
    // Storage: Staking Validators (r:1 w:0)
    // Storage: Staking CurrentEra (r:1 w:0)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Identity CurrentDid (r:1 w:0)
    fn unbond() -> Weight {
        // Minimum execution time: 126_635 nanoseconds.
        Weight::from_ref_time(138_766_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking Ledger (r:1 w:1)
    // Storage: Staking CurrentEra (r:1 w:0)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    /// The range of component `s` is `[0, 100]`.
    fn withdraw_unbonded_update(s: u32) -> Weight {
        // Minimum execution time: 115_097 nanoseconds.
        Weight::from_ref_time(132_677_063)
            // Standard Error: 46_491
            .saturating_add(Weight::from_ref_time(165_215).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking Ledger (r:1 w:1)
    // Storage: Staking CurrentEra (r:1 w:0)
    // Storage: Staking Bonded (r:1 w:1)
    // Storage: Staking SlashingSpans (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: Staking Validators (r:0 w:1)
    // Storage: Staking Payee (r:0 w:1)
    // Storage: Staking Nominators (r:0 w:1)
    // Storage: Staking SpanSlash (r:0 w:1)
    /// The range of component `s` is `[0, 100]`.
    fn withdraw_unbonded_kill(s: u32) -> Weight {
        // Minimum execution time: 145_425 nanoseconds.
        Weight::from_ref_time(189_057_188)
            // Standard Error: 114_473
            .saturating_add(Weight::from_ref_time(2_939_715).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(8))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(s.into())))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking Ledger (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Staking PermissionedIdentity (r:1 w:1)
    // Storage: Staking MinimumBondThreshold (r:1 w:0)
    // Storage: Staking ValidatorCommissionCap (r:1 w:0)
    // Storage: Staking Validators (r:1 w:1)
    // Storage: Identity AccountKeyRefCount (r:1 w:1)
    // Storage: Staking Nominators (r:0 w:1)
    fn validate() -> Weight {
        // Minimum execution time: 112_464 nanoseconds.
        Weight::from_ref_time(123_873_000)
            .saturating_add(DbWeight::get().reads(8))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Staking MinimumBondThreshold (r:0 w:1)
    fn set_min_bond_threshold() -> Weight {
        // Minimum execution time: 37_233 nanoseconds.
        Weight::from_ref_time(40_920_000).saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking PermissionedIdentity (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Instance2Group InactiveMembers (r:1 w:0)
    // Storage: Identity Claims (r:2 w:0)
    // Storage: Staking ValidatorCount (r:1 w:0)
    fn add_permissioned_validator() -> Weight {
        // Minimum execution time: 122_805 nanoseconds.
        Weight::from_ref_time(135_827_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking PermissionedIdentity (r:1 w:1)
    fn remove_permissioned_validator() -> Weight {
        // Minimum execution time: 71_096 nanoseconds.
        Weight::from_ref_time(76_292_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking ValidatorCommissionCap (r:1 w:1)
    // Storage: Staking Validators (r:2 w:1)
    /// The range of component `m` is `[0, 150]`.
    fn set_commission_cap(m: u32) -> Weight {
        // Minimum execution time: 71_194 nanoseconds.
        Weight::from_ref_time(63_687_828)
            // Standard Error: 384_151
            .saturating_add(Weight::from_ref_time(17_069_462).saturating_mul(m.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(m.into())))
            .saturating_add(DbWeight::get().writes(2))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(m.into())))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking Ledger (r:1 w:0)
    // Storage: Staking Nominators (r:1 w:1)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Instance2Group InactiveMembers (r:1 w:0)
    // Storage: Identity Claims (r:2 w:0)
    // Storage: Staking Validators (r:2 w:1)
    // Storage: Staking CurrentEra (r:1 w:0)
    /// The range of component `n` is `[1, 16]`.
    fn nominate(n: u32) -> Weight {
        // Minimum execution time: 173_399 nanoseconds.
        Weight::from_ref_time(195_267_443)
            // Standard Error: 380_167
            .saturating_add(Weight::from_ref_time(11_654_620).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(11))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking Ledger (r:1 w:0)
    // Storage: Staking Validators (r:1 w:1)
    // Storage: Staking Nominators (r:0 w:1)
    fn chill() -> Weight {
        // Minimum execution time: 68_203 nanoseconds.
        Weight::from_ref_time(74_852_000)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Staking Ledger (r:1 w:0)
    // Storage: Staking Payee (r:0 w:1)
    fn set_payee() -> Weight {
        // Minimum execution time: 52_084 nanoseconds.
        Weight::from_ref_time(64_108_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking Bonded (r:1 w:1)
    // Storage: Staking Ledger (r:2 w:2)
    fn set_controller() -> Weight {
        // Minimum execution time: 76_334 nanoseconds.
        Weight::from_ref_time(79_650_000)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Staking ValidatorCount (r:0 w:1)
    /// The range of component `c` is `[0, 1000]`.
    fn set_validator_count(c: u32) -> Weight {
        // Minimum execution time: 13_111 nanoseconds.
        Weight::from_ref_time(19_504_771)
            // Standard Error: 1_429
            .saturating_add(Weight::from_ref_time(741).saturating_mul(c.into()))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking ForceEra (r:0 w:1)
    fn force_no_eras() -> Weight {
        // Minimum execution time: 23_592 nanoseconds.
        Weight::from_ref_time(24_570_000).saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking ForceEra (r:0 w:1)
    fn force_new_era() -> Weight {
        // Minimum execution time: 15_073 nanoseconds.
        Weight::from_ref_time(16_408_000).saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking ForceEra (r:0 w:1)
    fn force_new_era_always() -> Weight {
        // Minimum execution time: 20_852 nanoseconds.
        Weight::from_ref_time(22_972_000).saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking Invulnerables (r:0 w:1)
    /// The range of component `v` is `[0, 1000]`.
    fn set_invulnerables(v: u32) -> Weight {
        // Minimum execution time: 16_472 nanoseconds.
        Weight::from_ref_time(29_155_178)
            // Standard Error: 3_120
            .saturating_add(Weight::from_ref_time(16_180).saturating_mul(v.into()))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking Bonded (r:1 w:1)
    // Storage: Staking SlashingSpans (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: Staking Ledger (r:0 w:1)
    // Storage: Staking Validators (r:0 w:1)
    // Storage: Staking Payee (r:0 w:1)
    // Storage: Staking Nominators (r:0 w:1)
    // Storage: Staking SpanSlash (r:0 w:1)
    /// The range of component `s` is `[0, 100]`.
    fn force_unstake(s: u32) -> Weight {
        // Minimum execution time: 105_211 nanoseconds.
        Weight::from_ref_time(114_063_628)
            // Standard Error: 100_021
            .saturating_add(Weight::from_ref_time(3_327_735).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(8))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(s.into())))
    }
    // Storage: Staking UnappliedSlashes (r:1 w:1)
    /// The range of component `s` is `[1, 1000]`.
    fn cancel_deferred_slash(s: u32) -> Weight {
        // Minimum execution time: 216_974 nanoseconds.
        Weight::from_ref_time(2_383_623_149)
            // Standard Error: 272_778
            .saturating_add(Weight::from_ref_time(12_853_979).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking CurrentEra (r:1 w:0)
    // Storage: Staking HistoryDepth (r:1 w:0)
    // Storage: Staking ErasValidatorReward (r:1 w:0)
    // Storage: Staking Bonded (r:2 w:0)
    // Storage: Staking Ledger (r:1 w:1)
    // Storage: Staking ErasStakersClipped (r:1 w:0)
    // Storage: Staking ErasRewardPoints (r:1 w:0)
    // Storage: Staking ErasValidatorPrefs (r:1 w:0)
    // Storage: Staking Payee (r:2 w:0)
    // Storage: System Account (r:3 w:3)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: Identity KeyRecords (r:4 w:0)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    /// The range of component `n` is `[1, 2048]`.
    fn payout_stakers(n: u32) -> Weight {
        // Minimum execution time: 5_243_594 nanoseconds.
        Weight::from_ref_time(8_386_997_613)
            // Standard Error: 643_446
            .saturating_add(Weight::from_ref_time(101_464_479).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(15))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(n.into())))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking CurrentEra (r:1 w:0)
    // Storage: Staking HistoryDepth (r:1 w:0)
    // Storage: Staking ErasValidatorReward (r:1 w:0)
    // Storage: Staking Bonded (r:2 w:0)
    // Storage: Staking Ledger (r:2 w:2)
    // Storage: Staking ErasStakersClipped (r:1 w:0)
    // Storage: Staking ErasRewardPoints (r:1 w:0)
    // Storage: Staking ErasValidatorPrefs (r:1 w:0)
    // Storage: Staking Payee (r:2 w:0)
    // Storage: System Account (r:3 w:3)
    // Storage: Balances Locks (r:2 w:2)
    // Storage: Identity KeyRecords (r:3 w:0)
    /// The range of component `n` is `[1, 2048]`.
    fn payout_stakers_alive_controller(n: u32) -> Weight {
        // Minimum execution time: 5_795_815 nanoseconds.
        Weight::from_ref_time(7_444_156_302)
            // Standard Error: 681_217
            .saturating_add(Weight::from_ref_time(119_317_643).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(15))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(n.into())))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking Ledger (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    /// The range of component `u` is `[0, 1000]`.
    /// The range of component `l` is `[1, 32]`.
    fn rebond(u: u32) -> Weight {
        // Minimum execution time: 47_949 nanoseconds.
        Weight::from_ref_time(49_739_043)
            // Standard Error: 5_104
            .saturating_add(Weight::from_ref_time(1_000).saturating_mul(u.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Staking CurrentEra (r:1 w:0)
    // Storage: Staking HistoryDepth (r:1 w:1)
    // Storage: Staking ErasStakersClipped (r:0 w:2)
    // Storage: Staking ErasValidatorPrefs (r:0 w:2)
    // Storage: Staking ErasValidatorReward (r:0 w:1)
    // Storage: Staking ErasRewardPoints (r:0 w:1)
    // Storage: Staking ErasStakers (r:0 w:2)
    // Storage: Staking ErasTotalStake (r:0 w:1)
    // Storage: Staking ErasStartSessionIndex (r:0 w:1)
    /// The range of component `e` is `[1, 100]`.
    fn set_history_depth(e: u32) -> Weight {
        // Minimum execution time: 87_301 nanoseconds.
        Weight::from_ref_time(87_459_000)
            // Standard Error: 294_772
            .saturating_add(Weight::from_ref_time(47_761_418).saturating_mul(e.into()))
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((7_u64).saturating_mul(e.into())))
    }
    // Storage: System Account (r:1 w:1)
    // Storage: Staking Bonded (r:1 w:1)
    // Storage: Staking SlashingSpans (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: Staking Ledger (r:0 w:1)
    // Storage: Staking Validators (r:0 w:1)
    // Storage: Staking Payee (r:0 w:1)
    // Storage: Staking Nominators (r:0 w:1)
    // Storage: Staking SpanSlash (r:0 w:1)
    /// The range of component `s` is `[1, 100]`.
    fn reap_stash(s: u32) -> Weight {
        // Minimum execution time: 128_718 nanoseconds.
        Weight::from_ref_time(135_800_816)
            // Standard Error: 141_964
            .saturating_add(Weight::from_ref_time(3_745_510).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(8))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(s.into())))
    }
    // Storage: Staking CurrentEra (r:1 w:1)
    // Storage: Staking HistoryDepth (r:1 w:0)
    // Storage: Staking QueuedElected (r:1 w:0)
    // Storage: Staking Validators (r:2 w:0)
    // Storage: Staking Bonded (r:101 w:0)
    // Storage: Staking Ledger (r:101 w:0)
    // Storage: Staking MinimumBondThreshold (r:1 w:0)
    // Storage: Identity KeyRecords (r:101 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Instance2Group InactiveMembers (r:1 w:0)
    // Storage: Identity Claims (r:202 w:0)
    // Storage: Staking PermissionedIdentity (r:1 w:0)
    // Storage: Staking Nominators (r:101 w:0)
    // Storage: Staking SlashingSpans (r:1 w:0)
    // Storage: Staking MinimumValidatorCount (r:1 w:0)
    // Storage: Staking ValidatorCount (r:1 w:0)
    // Storage: Staking ErasStakersClipped (r:0 w:1)
    // Storage: Staking ErasValidatorPrefs (r:0 w:1)
    // Storage: Staking SnapshotValidators (r:0 w:1)
    // Storage: Staking ErasStakers (r:0 w:1)
    // Storage: Staking ErasTotalStake (r:0 w:1)
    // Storage: Staking IsCurrentSessionFinal (r:0 w:1)
    // Storage: Staking ErasStartSessionIndex (r:0 w:1)
    // Storage: Staking QueuedScore (r:0 w:1)
    // Storage: Staking EraElectionStatus (r:0 w:1)
    // Storage: Staking SnapshotNominators (r:0 w:1)
    /// The range of component `v` is `[1, 10]`.
    /// The range of component `n` is `[1, 100]`.
    fn new_era(v: u32, n: u32) -> Weight {
        // Minimum execution time: 1_359_205 nanoseconds.
        Weight::from_ref_time(1_381_188_000)
            // Standard Error: 8_362_211
            .saturating_add(Weight::from_ref_time(174_542_417).saturating_mul(v.into()))
            // Standard Error: 858_956
            .saturating_add(Weight::from_ref_time(74_897_477).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(11))
            .saturating_add(DbWeight::get().reads((8_u64).saturating_mul(v.into())))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes(8))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(v.into())))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking CurrentEra (r:1 w:0)
    // Storage: Staking HistoryDepth (r:1 w:0)
    // Storage: Staking ErasValidatorReward (r:1 w:0)
    // Storage: Staking Bonded (r:101 w:0)
    // Storage: Staking Ledger (r:101 w:101)
    // Storage: Staking ErasStakersClipped (r:1 w:0)
    // Storage: Staking ErasRewardPoints (r:1 w:0)
    // Storage: Staking ErasValidatorPrefs (r:1 w:0)
    // Storage: Staking Payee (r:101 w:0)
    // Storage: System Account (r:102 w:102)
    // Storage: Balances Locks (r:101 w:101)
    // Storage: Identity KeyRecords (r:102 w:0)
    /// The range of component `v` is `[1, 10]`.
    /// The range of component `n` is `[1, 100]`.
    fn payout_all(v: u32, n: u32) -> Weight {
        // Minimum execution time: 2_051_915 nanoseconds.
        Weight::from_ref_time(2_095_207_000)
            // Standard Error: 86_088_735
            .saturating_add(Weight::from_ref_time(1_992_883_582).saturating_mul(v.into()))
            // Standard Error: 8_842_933
            .saturating_add(Weight::from_ref_time(252_432_365).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().reads((8_u64).saturating_mul(v.into())))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes(1))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(v.into())))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(n.into())))
    }
    // Storage: Staking Bonded (r:1 w:0)
    // Storage: Staking Ledger (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    /// The range of component `l` is `[1, 32]`.
    fn do_slash(_l: u32) -> Weight {
        // Minimum execution time: 122_815 nanoseconds.
        Weight::from_ref_time(140_228_092)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking CurrentEra (r:1 w:0)
    // Storage: Staking QueuedScore (r:1 w:1)
    // Storage: Staking SnapshotValidators (r:1 w:0)
    // Storage: Staking ValidatorCount (r:1 w:0)
    // Storage: Staking SnapshotNominators (r:1 w:0)
    // Storage: Staking Validators (r:500 w:0)
    // Storage: Staking Nominators (r:500 w:0)
    // Storage: Staking SlashingSpans (r:100 w:0)
    // Storage: Staking Bonded (r:500 w:0)
    // Storage: Staking Ledger (r:500 w:0)
    // Storage: Staking QueuedElected (r:0 w:1)
    /// The range of component `v` is `[1000, 2000]`.
    /// The range of component `n` is `[1000, 2000]`.
    /// The range of component `a` is `[200, 500]`.
    /// The range of component `w` is `[16, 100]`.
    fn submit_solution_better(_v: u32, _n: u32, a: u32, w: u32) -> Weight {
        // Minimum execution time: 27_809_354 nanoseconds.
        Weight::from_ref_time(28_545_925_000)
            // Standard Error: 677_240
            .saturating_add(Weight::from_ref_time(61_488_150).saturating_mul(a.into()))
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().reads((4_u64).saturating_mul(a.into())))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(w.into())))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Staking SlashingAllowedFor (r:0 w:1)
    fn change_slashing_allowed_for() -> Weight {
        // Minimum execution time: 39_993 nanoseconds.
        Weight::from_ref_time(45_913_000).saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking ValidatorCount (r:1 w:0)
    // Storage: Staking PermissionedIdentity (r:1 w:1)
    fn update_permissioned_validator_intended_count() -> Weight {
        // Minimum execution time: 61_762 nanoseconds.
        Weight::from_ref_time(68_758_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking ValidatorCount (r:1 w:1)
    fn increase_validator_count() -> Weight {
        // Minimum execution time: 32_520 nanoseconds.
        Weight::from_ref_time(35_224_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking ValidatorCount (r:1 w:1)
    fn scale_validator_count() -> Weight {
        // Minimum execution time: 31_101 nanoseconds.
        Weight::from_ref_time(32_870_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Staking EraElectionStatus (r:1 w:0)
    // Storage: Staking PermissionedIdentity (r:1 w:1)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity IsDidFrozen (r:1 w:0)
    // Storage: Staking Validators (r:1 w:1)
    // Storage: Identity AccountKeyRefCount (r:1 w:1)
    // Storage: Staking Nominators (r:0 w:1)
    /// The range of component `s` is `[1, 100]`.
    fn chill_from_governance(s: u32) -> Weight {
        // Minimum execution time: 123_246 nanoseconds.
        Weight::from_ref_time(128_352_833)
            // Standard Error: 521_546
            .saturating_add(Weight::from_ref_time(37_240_472).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().reads((3_u64).saturating_mul(s.into())))
            .saturating_add(DbWeight::get().writes(1))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(s.into())))
    }
}
