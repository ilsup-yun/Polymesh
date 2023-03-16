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

//! Autogenerated weights for pallet_settlement
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-29, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512
//! HOSTNAME: `dev-fsn001`, CPU: `AMD Ryzen 9 5950X 16-Core Processor`

// Executed Command:
// ./target/release/polymesh
// benchmark
// pallet
// -s
// 100
// -r
// 5
// -p=pallet_settlement
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

/// Weights for pallet_settlement using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_settlement::WeightInfo for SubstrateWeight {
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueCounter (r:1 w:1)
    // Storage: Settlement UserVenues (r:1 w:1)
    // Storage: Settlement VenueInfo (r:0 w:1)
    // Storage: Settlement Details (r:0 w:1)
    // Storage: Settlement VenueSigners (r:0 w:50)
    /// The range of component `d` is `[1, 2048]`.
    /// The range of component `s` is `[0, 50]`.
    fn create_venue(d: u32, s: u32) -> Weight {
        // Minimum execution time: 39_733 nanoseconds.
        Weight::from_ref_time(52_938_000)
            // Standard Error: 202
            .saturating_add(Weight::from_ref_time(17_000).saturating_mul(d.into()))
            // Standard Error: 8_239
            .saturating_add(Weight::from_ref_time(3_425_000).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(s.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement Details (r:0 w:1)
    /// The range of component `d` is `[1, 2048]`.
    fn update_venue_details(d: u32) -> Weight {
        // Minimum execution time: 32_680 nanoseconds.
        Weight::from_ref_time(53_526_000)
            // Standard Error: 288
            .saturating_add(Weight::from_ref_time(6_000).saturating_mul(d.into()))
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:1)
    fn update_venue_type() -> Weight {
        // Minimum execution time: 32_370 nanoseconds.
        Weight::from_ref_time(46_711_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement VenueSigners (r:1 w:1)
    /// The range of component `s` is `[0, 50]`.
    fn update_venue_signers(s: u32) -> Weight {
        // Minimum execution time: 30_626 nanoseconds.
        Weight::from_ref_time(65_731_000)
            // Standard Error: 10_259
            .saturating_add(Weight::from_ref_time(8_300_000).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(s.into())))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(s.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement VenueFiltering (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:0 w:1)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    /// The range of component `l` is `[1, 10]`.
    fn add_instruction(l: u32) -> Weight {
        // Minimum execution time: 43_019 nanoseconds.
        Weight::from_ref_time(76_871_000)
            // Standard Error: 45_281
            .saturating_add(Weight::from_ref_time(16_619_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(1))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement VenueFiltering (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:0 w:1)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    /// The range of component `l` is `[1, 10]`.
    fn add_instruction_with_settle_on_block_type(l: u32) -> Weight {
        // Minimum execution time: 61_073 nanoseconds.
        Weight::from_ref_time(87_746_000)
            // Standard Error: 36_316
            .saturating_add(Weight::from_ref_time(20_841_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(3))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement VenueFiltering (r:1 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement InstructionLegsV2 (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:2 w:1)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    // Storage: Settlement InstructionLegStatus (r:0 w:1)
    /// The range of component `l` is `[1, 10]`.
    fn add_and_affirm_instruction(l: u32) -> Weight {
        // Minimum execution time: 110_675 nanoseconds.
        Weight::from_ref_time(169_005_000)
            // Standard Error: 97_715
            .saturating_add(Weight::from_ref_time(72_527_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(5))
            .saturating_add(DbWeight::get().writes((6_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement VenueFiltering (r:1 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement InstructionLegsV2 (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:2 w:1)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    // Storage: Settlement InstructionLegStatus (r:0 w:1)
    /// The range of component `l` is `[1, 10]`.
    fn add_and_affirm_instruction_with_settle_on_block_type(l: u32) -> Weight {
        // Minimum execution time: 132_565 nanoseconds.
        Weight::from_ref_time(201_203_000)
            // Standard Error: 58_630
            .saturating_add(Weight::from_ref_time(77_187_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(9))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(7))
            .saturating_add(DbWeight::get().writes((6_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Settlement VenueFiltering (r:0 w:1)
    fn set_venue_filtering() -> Weight {
        // Minimum execution time: 40_985 nanoseconds.
        Weight::from_ref_time(98_394_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Settlement VenueAllowList (r:0 w:1)
    /// The range of component `v` is `[0, 100]`.
    fn allow_venues(v: u32) -> Weight {
        // Minimum execution time: 39_272 nanoseconds.
        Weight::from_ref_time(69_133_000)
            // Standard Error: 17_875
            .saturating_add(Weight::from_ref_time(2_721_000).saturating_mul(v.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(v.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Settlement VenueAllowList (r:0 w:1)
    /// The range of component `v` is `[0, 100]`.
    fn disallow_venues(v: u32) -> Weight {
        // Minimum execution time: 38_962 nanoseconds.
        Weight::from_ref_time(61_446_000)
            // Standard Error: 2_619
            .saturating_add(Weight::from_ref_time(2_674_000).saturating_mul(v.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(v.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:0)
    // Storage: Settlement InstructionLegsV2 (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:1 w:0)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Scheduler Lookup (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement UserAffirmations (r:1 w:1)
    // Storage: Settlement InstructionLegStatus (r:1 w:1)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    /// The range of component `l` is `[0, 10]`.
    fn withdraw_affirmation(l: u32) -> Weight {
        // Minimum execution time: 33_151 nanoseconds.
        Weight::from_ref_time(77_041_000)
            // Standard Error: 119_212
            .saturating_add(Weight::from_ref_time(63_629_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().reads((5_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(1))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement InstructionLegsV2 (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:2 w:1)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement InstructionLegStatus (r:1 w:1)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Scheduler Lookup (r:1 w:0)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    /// The range of component `l` is `[1, 10]`.
    fn reject_instruction(l: u32) -> Weight {
        // Minimum execution time: 94_174 nanoseconds.
        Weight::from_ref_time(151_533_000)
            // Standard Error: 94_753
            .saturating_add(Weight::from_ref_time(48_965_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().reads((3_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(3))
            .saturating_add(DbWeight::get().writes((6_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:0)
    // Storage: Settlement InstructionLegsV2 (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:1 w:0)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Scheduler Lookup (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement UserAffirmations (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    /// The range of component `l` is `[0, 10]`.
    fn affirm_instruction(l: u32) -> Weight {
        // Minimum execution time: 52_097 nanoseconds.
        Weight::from_ref_time(130_307_000)
            // Standard Error: 109_941
            .saturating_add(Weight::from_ref_time(39_729_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(8))
            .saturating_add(DbWeight::get().reads((3_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(3))
            .saturating_add(DbWeight::get().writes((2_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement UserAffirmations (r:1 w:1)
    // Storage: Settlement VenueSigners (r:1 w:0)
    // Storage: Settlement ReceiptsUsed (r:1 w:1)
    // Storage: Settlement InstructionLegsV2 (r:2 w:0)
    // Storage: Settlement InstructionLegs (r:2 w:0)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    // Storage: Settlement InstructionLegStatus (r:0 w:1)
    /// The range of component `r` is `[1, 10]`.
    fn affirm_with_receipts(r: u32) -> Weight {
        // Minimum execution time: 135_991 nanoseconds.
        Weight::from_ref_time(156_039_000)
            // Standard Error: 104_920
            .saturating_add(Weight::from_ref_time(168_230_000).saturating_mul(r.into()))
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(r.into())))
            .saturating_add(DbWeight::get().writes(2))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(r.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement ReceiptsUsed (r:0 w:1)
    fn change_receipt_validity() -> Weight {
        // Minimum execution time: 28_082 nanoseconds.
        Weight::from_ref_time(42_992_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement InstructionLegs (r:1 w:0)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement VenueFiltering (r:1 w:0)
    // Storage: Settlement InstructionLegStatus (r:1 w:1)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Asset Frozen (r:1 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity Claims (r:52 w:0)
    // Storage: Portfolio Portfolios (r:2 w:0)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:2 w:2)
    // Storage: Asset ScopeIdOf (r:2 w:0)
    // Storage: Asset AggregateBalance (r:2 w:2)
    // Storage: Statistics AssetTransferCompliances (r:1 w:0)
    // Storage: Statistics AssetStats (r:1 w:1)
    // Storage: Statistics TransferConditionExemptEntities (r:1 w:0)
    // Storage: ComplianceManager AssetCompliances (r:1 w:0)
    // Storage: Asset BalanceOf (r:2 w:2)
    // Storage: Checkpoint Schedules (r:1 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Storage: Portfolio PortfolioAssetCount (r:2 w:2)
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement AffirmsReceived (r:0 w:2)
    // Storage: Asset BalanceOfAtScope (r:0 w:2)
    /// The range of component `l` is `[0, 10]`.
    fn execute_scheduled_instruction(l: u32) -> Weight {
        // Minimum execution time: 49_561 nanoseconds.
        Weight::from_ref_time(169_535_200)
            // Standard Error: 1_232_382
            .saturating_add(Weight::from_ref_time(405_339_852).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(36))
            .saturating_add(DbWeight::get().reads((31_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((16_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement InstructionLegsV2 (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:11 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn reschedule_instruction() -> Weight {
        // Minimum execution time: 128_167 nanoseconds.
        Weight::from_ref_time(315_940_000)
            .saturating_add(DbWeight::get().reads(16))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionMemos (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement VenueFiltering (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:0 w:1)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    /// The range of component `l` is `[1, 10]`.
    fn add_instruction_with_memo_and_settle_on_block_type(l: u32) -> Weight {
        // Minimum execution time: 59_731 nanoseconds.
        Weight::from_ref_time(111_770_000)
            // Standard Error: 43_109
            .saturating_add(Weight::from_ref_time(17_031_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement VenueFiltering (r:1 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement InstructionLegsV2 (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:2 w:1)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionMemos (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    // Storage: Settlement InstructionLegStatus (r:0 w:1)
    /// The range of component `l` is `[1, 10]`.
    fn add_and_affirm_instruction_with_memo_and_settle_on_block_type(l: u32) -> Weight {
        // Minimum execution time: 129_519 nanoseconds.
        Weight::from_ref_time(268_751_000)
            // Standard Error: 70_208
            .saturating_add(Weight::from_ref_time(72_872_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(97))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(8))
            .saturating_add(DbWeight::get().writes((6_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement InstructionLegsV2 (r:1 w:0)
    // Storage: Settlement InstructionLegs (r:2 w:1)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement VenueFiltering (r:1 w:0)
    // Storage: Settlement InstructionLegStatus (r:1 w:1)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Asset Frozen (r:1 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Portfolio Portfolios (r:2 w:0)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:2 w:2)
    // Storage: Asset AggregateBalance (r:2 w:2)
    // Storage: Statistics AssetTransferCompliances (r:1 w:0)
    // Storage: Statistics AssetStats (r:1 w:1)
    // Storage: Statistics TransferConditionExemptEntities (r:1 w:0)
    // Storage: ComplianceManager AssetCompliances (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity Claims (r:50 w:0)
    // Storage: Asset BalanceOf (r:2 w:2)
    // Storage: Checkpoint Schedules (r:1 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Storage: Portfolio PortfolioAssetCount (r:2 w:2)
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:2)
    // Storage: Asset BalanceOfAtScope (r:0 w:2)
    /// The range of component `l` is `[1, 10]`.
    fn execute_manual_instruction(l: u32) -> Weight {
        // Minimum execution time: 579_329 nanoseconds.
        Weight::from_ref_time(447_734_000)
            // Standard Error: 447_198
            .saturating_add(Weight::from_ref_time(783_783_000).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(58))
            .saturating_add(DbWeight::get().reads((24_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(6))
            .saturating_add(DbWeight::get().writes((16_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement VenueFiltering (r:2 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement InstructionLegsV2 (r:0 w:11)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionMemos (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    fn add_instruction_with_memo_v2(f: u32) -> Weight {
        Weight::from_ref_time(205_603_000 as u64)
            // Standard Error: 191_000
            .saturating_add(Weight::from_ref_time(7_185_000 as u64).saturating_mul(f as u64))
            .saturating_add(DbWeight::get().reads(8 as u64))
            .saturating_add(DbWeight::get().writes(19 as u64))
            .saturating_add(DbWeight::get().writes((1 as u64).saturating_mul(f as u64)))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement VenueFiltering (r:2 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement InstructionLegsV2 (r:12 w:11)
    // Storage: Portfolio PortfolioNFT (r:100 w:0)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionMemos (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    // Storage: Settlement InstructionLegStatus (r:0 w:11)
    fn add_and_affirm_instruction_with_memo_v2(f: u32, n: u32) -> Weight {
        Weight::from_ref_time(326_951_000 as u64)
            // Standard Error: 356_000
            .saturating_add(Weight::from_ref_time(21_128_000 as u64).saturating_mul(f as u64))
            // Standard Error: 17_000
            .saturating_add(Weight::from_ref_time(20_379_000 as u64).saturating_mul(n as u64))
            .saturating_add(DbWeight::get().reads(14 as u64))
            .saturating_add(DbWeight::get().reads((1 as u64).saturating_mul(f as u64)))
            .saturating_add(DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
            .saturating_add(DbWeight::get().writes(12 as u64))
            .saturating_add(DbWeight::get().writes((2 as u64).saturating_mul(f as u64)))
            .saturating_add(DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement UserAffirmations (r:1 w:1)
    // Storage: Settlement InstructionLegsV2 (r:12 w:0)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Portfolio PortfolioNFT (r:100 w:0)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    // Storage: Settlement InstructionLegStatus (r:0 w:11)
    fn affirm_instruction_v2(f: u32, n: u32) -> Weight {
        Weight::from_ref_time(19_543_000 as u64)
            // Standard Error: 360_000
            .saturating_add(Weight::from_ref_time(46_742_000 as u64).saturating_mul(f as u64))
            // Standard Error: 18_000
            .saturating_add(Weight::from_ref_time(20_305_000 as u64).saturating_mul(n as u64))
            .saturating_add(DbWeight::get().reads(10 as u64))
            .saturating_add(DbWeight::get().reads((1 as u64).saturating_mul(f as u64)))
            .saturating_add(DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
            .saturating_add(DbWeight::get().writes(5 as u64))
            .saturating_add(DbWeight::get().writes((1 as u64).saturating_mul(f as u64)))
            .saturating_add(DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement UserAffirmations (r:1 w:1)
    // Storage: Settlement InstructionLegsV2 (r:12 w:0)
    // Storage: Settlement InstructionLegStatus (r:11 w:11)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    fn withdraw_affirmation_v2(f: u32, n: u32) -> Weight {
        Weight::from_ref_time(184_720_000 as u64)
            // Standard Error: 290_000
            .saturating_add(Weight::from_ref_time(23_626_000 as u64).saturating_mul(f as u64))
            // Standard Error: 14_000
            .saturating_add(Weight::from_ref_time(14_834_000 as u64).saturating_mul(n as u64))
            .saturating_add(DbWeight::get().reads(8 as u64))
            .saturating_add(DbWeight::get().reads((2 as u64).saturating_mul(f as u64)))
            .saturating_add(DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
            .saturating_add(DbWeight::get().writes(5 as u64))
            .saturating_add(DbWeight::get().writes((1 as u64).saturating_mul(f as u64)))
            .saturating_add(DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
    }
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement InstructionLegsV2 (r:12 w:11)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement InstructionLegStatus (r:11 w:11)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:1)
    fn reject_instruction_v2(f: u32, n: u32) -> Weight {
        Weight::from_ref_time(86_638_000 as u64)
            // Standard Error: 365_000
            .saturating_add(Weight::from_ref_time(46_128_000 as u64).saturating_mul(f as u64))
            // Standard Error: 18_000
            .saturating_add(Weight::from_ref_time(16_393_000 as u64).saturating_mul(n as u64))
            .saturating_add(DbWeight::get().reads(8 as u64))
            .saturating_add(DbWeight::get().reads((2 as u64).saturating_mul(f as u64)))
            .saturating_add(DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
            .saturating_add(DbWeight::get().writes(10 as u64))
            .saturating_add(DbWeight::get().writes((2 as u64).saturating_mul(f as u64)))
            .saturating_add(DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
    }
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement InstructionLegsV2 (r:102 w:101)
    // Storage: Settlement VenueFiltering (r:101 w:0)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Settlement InstructionLegStatus (r:101 w:101)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Asset Frozen (r:101 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Portfolio Portfolios (r:2 w:0)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:2 w:2)
    // Storage: Asset AggregateBalance (r:2 w:2)
    // Storage: Statistics AssetTransferCompliances (r:1 w:0)
    // Storage: Statistics AssetStats (r:1 w:1)
    // Storage: Statistics TransferConditionExemptEntities (r:1 w:0)
    // Storage: ComplianceManager AssetCompliances (r:101 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity Claims (r:50 w:0)
    // Storage: Asset BalanceOf (r:2 w:2)
    // Storage: Checkpoint Schedules (r:1 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Storage: Portfolio PortfolioAssetCount (r:2 w:2)
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Storage: NFT CollectionTicker (r:100 w:0)
    // Storage: NFT NumberOfNFTs (r:200 w:200)
    // Storage: Portfolio PortfolioNFT (r:100 w:200)
    // Storage: Settlement UserAffirmations (r:0 w:2)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:2)
    // Storage: Asset BalanceOfAtScope (r:0 w:2)
    fn execute_scheduled_instruction(f: u32, n: u32) -> Weight {
        (3_907_651_000 as Weight)
            // Standard Error: 183_402_000
            .saturating_add((557_992_000 as Weight).saturating_mul(f as Weight))
            // Standard Error: 9_215_000
            .saturating_add((627_229_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(DbWeight::get().reads(59 as Weight))
            .saturating_add(DbWeight::get().reads((20 as Weight).saturating_mul(f as Weight)))
            .saturating_add(DbWeight::get().reads((10 as Weight).saturating_mul(n as Weight)))
            .saturating_add(DbWeight::get().writes(12 as Weight))
            .saturating_add(DbWeight::get().writes((10 as Weight).saturating_mul(f as Weight)))
            .saturating_add(DbWeight::get().writes((7 as Weight).saturating_mul(n as Weight)))
    }
}
