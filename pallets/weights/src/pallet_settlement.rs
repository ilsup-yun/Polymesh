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
//! DATE: 2023-01-26, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
    // Storage: Settlement VenueSigners (r:0 w:49)
    /// The range of component `d` is `[1, 2048]`.
    /// The range of component `s` is `[0, 50]`.
    fn create_venue(d: u32, s: u32) -> Weight {
        // Minimum execution time: 52_214 nanoseconds.
        Weight::from_ref_time(56_791_349)
            // Standard Error: 336
            .saturating_add(Weight::from_ref_time(2_591).saturating_mul(d.into()))
            // Standard Error: 13_687
            .saturating_add(Weight::from_ref_time(2_483_094).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(s.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement Details (r:0 w:1)
    /// The range of component `d` is `[1, 2048]`.
    fn update_venue_details(d: u32) -> Weight {
        // Minimum execution time: 43_431 nanoseconds.
        Weight::from_ref_time(44_527_014)
            // Standard Error: 70
            .saturating_add(Weight::from_ref_time(2_133).saturating_mul(d.into()))
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:1)
    fn update_venue_type() -> Weight {
        // Minimum execution time: 42_337 nanoseconds.
        Weight::from_ref_time(43_003_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement VenueSigners (r:49 w:49)
    /// The range of component `s` is `[0, 50]`.
    fn update_venue_signers(s: u32) -> Weight {
        // Minimum execution time: 39_682 nanoseconds.
        Weight::from_ref_time(48_882_401)
            // Standard Error: 12_265
            .saturating_add(Weight::from_ref_time(5_293_029).saturating_mul(s.into()))
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(s.into())))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(s.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Settlement VenueFiltering (r:0 w:1)
    fn set_venue_filtering() -> Weight {
        // Minimum execution time: 48_632 nanoseconds.
        Weight::from_ref_time(48_978_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Settlement VenueAllowList (r:0 w:99)
    /// The range of component `v` is `[0, 100]`.
    fn allow_venues(v: u32) -> Weight {
        // Minimum execution time: 45_984 nanoseconds.
        Weight::from_ref_time(47_003_906)
            // Standard Error: 3_958
            .saturating_add(Weight::from_ref_time(2_687_051).saturating_mul(v.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(v.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Settlement VenueAllowList (r:0 w:99)
    /// The range of component `v` is `[0, 100]`.
    fn disallow_venues(v: u32) -> Weight {
        // Minimum execution time: 46_100 nanoseconds.
        Weight::from_ref_time(49_169_984)
            // Standard Error: 4_352
            .saturating_add(Weight::from_ref_time(2_577_382).saturating_mul(v.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(v.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:0)
    // Storage: Settlement InstructionStatuses (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:110 w:0)
    // Storage: Settlement UserAffirmations (r:110 w:110)
    // Storage: Settlement VenueSigners (r:1 w:0)
    // Storage: Settlement ReceiptsUsed (r:10 w:10)
    // Storage: Settlement InstructionLegs (r:121 w:0)
    // Storage: Portfolio PortfolioNFT (r:100 w:0)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Asset Tokens (r:10 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:10 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:10 w:10)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement OffChainAffirmations (r:0 w:10)
    // Storage: Settlement AffirmsReceived (r:0 w:110)
    // Storage: Settlement InstructionLegStatus (r:0 w:120)
    /// The range of component `f` is `[1, 10]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `o` is `[0, 10]`.
    fn affirm_with_receipts(f: u32, n: u32, o: u32) -> Weight {
        // Minimum execution time: 1_408_630 nanoseconds.
        Weight::from_ref_time(143_063_076)
            // Standard Error: 485_525
            .saturating_add(Weight::from_ref_time(41_729_458).saturating_mul(f.into()))
            // Standard Error: 44_912
            .saturating_add(Weight::from_ref_time(44_694_818).saturating_mul(n.into()))
            // Standard Error: 439_810
            .saturating_add(Weight::from_ref_time(85_208_035).saturating_mul(o.into()))
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().reads((5_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().reads((2_u64).saturating_mul(o.into())))
            .saturating_add(DbWeight::get().writes(1))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(o.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement InstructionStatuses (r:1 w:1)
    // Storage: Settlement InstructionLegs (r:121 w:120)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement VenueFiltering (r:120 w:0)
    // Storage: Settlement InstructionMemos (r:1 w:0)
    // Storage: Settlement InstructionLegStatus (r:120 w:120)
    // Storage: Portfolio PortfolioLockedAssets (r:10 w:10)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Asset Frozen (r:110 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:10 w:0)
    // Storage: Portfolio Portfolios (r:20 w:0)
    // Storage: Asset Tokens (r:10 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:20 w:20)
    // Storage: Asset AggregateBalance (r:20 w:20)
    // Storage: Statistics AssetTransferCompliances (r:10 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity Claims (r:201 w:0)
    // Storage: Statistics AssetStats (r:140 w:100)
    // Storage: ComplianceManager AssetCompliances (r:110 w:0)
    // Storage: Asset BalanceOf (r:20 w:20)
    // Storage: Checkpoint Schedules (r:10 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:10 w:0)
    // Storage: Portfolio PortfolioAssetCount (r:10 w:10)
    // Storage: Statistics ActiveAssetStats (r:10 w:0)
    // Storage: NFT CollectionTicker (r:100 w:0)
    // Storage: NFT NumberOfNFTs (r:200 w:200)
    // Storage: Portfolio PortfolioNFT (r:100 w:200)
    // Storage: Settlement OffChainAffirmations (r:10 w:10)
    // Storage: Settlement AffirmsReceived (r:220 w:220)
    // Storage: Settlement UserAffirmations (r:0 w:220)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Asset BalanceOfAtScope (r:0 w:2)
    /// The range of component `f` is `[1, 10]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `o` is `[0, 10]`.
    fn execute_manual_instruction(f: u32, n: u32, o: u32) -> Weight {
        // Minimum execution time: 5_146_933 nanoseconds.
        Weight::from_ref_time(5_157_415_000)
            // Standard Error: 5_181_166
            .saturating_add(Weight::from_ref_time(269_622_397).saturating_mul(f.into()))
            // Standard Error: 500_277
            .saturating_add(Weight::from_ref_time(137_667_763).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(9))
            .saturating_add(DbWeight::get().reads((57_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().reads((12_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().reads((4_u64).saturating_mul(o.into())))
            .saturating_add(DbWeight::get().writes(6))
            .saturating_add(DbWeight::get().writes((24_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().writes((11_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(o.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Asset Tokens (r:120 w:0)
    // Storage: Settlement VenueFiltering (r:120 w:0)
    // Storage: Portfolio PortfolioCustodian (r:110 w:0)
    // Storage: Asset TickersExemptFromAffirmation (r:110 w:0)
    // Storage: Asset PreApprovedTicker (r:110 w:0)
    // Storage: Portfolio PreApprovedPortfolios (r:110 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Settlement InstructionLegs (r:0 w:120)
    // Storage: Settlement UserAffirmations (r:0 w:220)
    // Storage: Settlement OffChainAffirmations (r:0 w:10)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionMemos (r:0 w:1)
    // Storage: Settlement InstructionStatuses (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    /// The range of component `f` is `[1, 10]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `o` is `[0, 10]`.
    fn add_instruction(f: u32, n: u32, o: u32) -> Weight {
        // Minimum execution time: 491_379 nanoseconds.
        Weight::from_ref_time(98_250_191)
            // Standard Error: 548_968
            .saturating_add(Weight::from_ref_time(31_107_274).saturating_mul(f.into()))
            // Standard Error: 50_781
            .saturating_add(Weight::from_ref_time(28_282_332).saturating_mul(n.into()))
            // Standard Error: 497_281
            .saturating_add(Weight::from_ref_time(8_802_687).saturating_mul(o.into()))
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().reads((2_u64).saturating_mul(o.into())))
            .saturating_add(DbWeight::get().writes(8))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes((2_u64).saturating_mul(o.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement VenueInfo (r:1 w:0)
    // Storage: Asset Tokens (r:120 w:0)
    // Storage: Settlement VenueFiltering (r:120 w:0)
    // Storage: Portfolio PortfolioCustodian (r:220 w:0)
    // Storage: Asset TickersExemptFromAffirmation (r:110 w:0)
    // Storage: Asset PreApprovedTicker (r:110 w:0)
    // Storage: Portfolio PreApprovedPortfolios (r:110 w:0)
    // Storage: Settlement InstructionCounter (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Settlement InstructionLegs (r:121 w:120)
    // Storage: Portfolio PortfolioNFT (r:100 w:0)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Portfolio PortfolioAssetBalances (r:10 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:10 w:10)
    // Storage: Settlement UserAffirmations (r:0 w:220)
    // Storage: Settlement OffChainAffirmations (r:0 w:10)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement InstructionMemos (r:0 w:1)
    // Storage: Settlement InstructionStatuses (r:0 w:1)
    // Storage: Settlement InstructionDetails (r:0 w:1)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:110)
    // Storage: Settlement InstructionLegStatus (r:0 w:110)
    /// The range of component `f` is `[1, 10]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `o` is `[0, 10]`.
    fn add_and_affirm_instruction(f: u32, n: u32, o: u32) -> Weight {
        // Minimum execution time: 1_018_734 nanoseconds.
        Weight::from_ref_time(90_489_356)
            // Standard Error: 1_450_841
            .saturating_add(Weight::from_ref_time(76_475_324).saturating_mul(f.into()))
            // Standard Error: 134_206
            .saturating_add(Weight::from_ref_time(68_877_952).saturating_mul(n.into()))
            // Standard Error: 1_314_238
            .saturating_add(Weight::from_ref_time(18_040_837).saturating_mul(o.into()))
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().reads((10_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().reads((10_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().reads((3_u64).saturating_mul(o.into())))
            .saturating_add(DbWeight::get().writes(8))
            .saturating_add(DbWeight::get().writes((6_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().writes((6_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes((2_u64).saturating_mul(o.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:0)
    // Storage: Settlement InstructionStatuses (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:110 w:0)
    // Storage: Settlement UserAffirmations (r:110 w:110)
    // Storage: Settlement InstructionLegs (r:121 w:0)
    // Storage: Portfolio PortfolioNFT (r:100 w:0)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Asset Tokens (r:10 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:10 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:10 w:10)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:110)
    // Storage: Settlement InstructionLegStatus (r:0 w:110)
    /// The range of component `f` is `[1, 10]`.
    /// The range of component `n` is `[1, 100]`.
    fn affirm_instruction(f: u32, n: u32) -> Weight {
        // Minimum execution time: 664_841 nanoseconds.
        Weight::from_ref_time(184_931_066)
            // Standard Error: 666_144
            .saturating_add(Weight::from_ref_time(46_538_546).saturating_mul(f.into()))
            // Standard Error: 62_596
            .saturating_add(Weight::from_ref_time(44_438_784).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(15))
            .saturating_add(DbWeight::get().reads((6_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().reads((5_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes(1))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(n.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Settlement InstructionDetails (r:1 w:0)
    // Storage: Settlement InstructionStatuses (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:110 w:0)
    // Storage: Settlement UserAffirmations (r:110 w:110)
    // Storage: Settlement InstructionLegs (r:121 w:0)
    // Storage: Settlement InstructionLegStatus (r:110 w:110)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Portfolio PortfolioLockedAssets (r:10 w:10)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement AffirmsReceived (r:0 w:110)
    /// The range of component `f` is `[1, 10]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `o` is `[0, 10]`.
    fn withdraw_affirmation(f: u32, n: u32, o: u32) -> Weight {
        // Minimum execution time: 548_872 nanoseconds.
        Weight::from_ref_time(66_205_878)
            // Standard Error: 675_337
            .saturating_add(Weight::from_ref_time(40_834_180).saturating_mul(f.into()))
            // Standard Error: 62_470
            .saturating_add(Weight::from_ref_time(45_360_749).saturating_mul(n.into()))
            // Standard Error: 611_751
            .saturating_add(Weight::from_ref_time(10_447_485).saturating_mul(o.into()))
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().reads((5_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().reads((5_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(o.into())))
            .saturating_add(DbWeight::get().writes(1))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(n.into())))
    }
    // Storage: Settlement InstructionStatuses (r:1 w:1)
    // Storage: Settlement InstructionLegs (r:121 w:120)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Settlement InstructionLegStatus (r:120 w:120)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Portfolio PortfolioLockedAssets (r:10 w:10)
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement OffChainAffirmations (r:10 w:10)
    // Storage: Settlement AffirmsReceived (r:220 w:220)
    // Storage: Settlement UserAffirmations (r:0 w:220)
    // Storage: Settlement InstructionAffirmsPending (r:0 w:1)
    // Storage: Settlement ReceiptsUsed (r:0 w:10)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    /// The range of component `f` is `[1, 10]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `o` is `[0, 10]`.
    fn reject_instruction(f: u32, n: u32, o: u32) -> Weight {
        // Minimum execution time: 792_478 nanoseconds.
        Weight::from_ref_time(160_371_505)
            // Standard Error: 1_130_360
            .saturating_add(Weight::from_ref_time(27_741_381).saturating_mul(f.into()))
            // Standard Error: 104_561
            .saturating_add(Weight::from_ref_time(41_082_059).saturating_mul(n.into()))
            // Standard Error: 1_023_932
            .saturating_add(Weight::from_ref_time(25_455_113).saturating_mul(o.into()))
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().reads((5_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().reads((5_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().reads((3_u64).saturating_mul(o.into())))
            .saturating_add(DbWeight::get().writes(6))
            .saturating_add(DbWeight::get().writes((7_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().writes((7_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes((4_u64).saturating_mul(o.into())))
    }
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement InstructionStatuses (r:1 w:1)
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement InstructionLegs (r:121 w:120)
    // Storage: Settlement VenueFiltering (r:120 w:0)
    // Storage: Settlement InstructionMemos (r:1 w:0)
    // Storage: Settlement InstructionLegStatus (r:120 w:120)
    // Storage: Portfolio PortfolioLockedAssets (r:10 w:10)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Asset Frozen (r:110 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:10 w:0)
    // Storage: Portfolio Portfolios (r:20 w:0)
    // Storage: Asset Tokens (r:10 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:20 w:20)
    // Storage: Asset AggregateBalance (r:20 w:20)
    // Storage: Statistics AssetTransferCompliances (r:10 w:0)
    // Storage: ComplianceManager AssetCompliances (r:110 w:0)
    // Storage: Asset BalanceOf (r:20 w:20)
    // Storage: Checkpoint Schedules (r:10 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:10 w:0)
    // Storage: Portfolio PortfolioAssetCount (r:10 w:10)
    // Storage: Statistics ActiveAssetStats (r:10 w:0)
    // Storage: NFT CollectionTicker (r:100 w:0)
    // Storage: NFT NumberOfNFTs (r:200 w:200)
    // Storage: Portfolio PortfolioNFT (r:100 w:200)
    // Storage: Settlement OffChainAffirmations (r:10 w:10)
    // Storage: Settlement AffirmsReceived (r:220 w:220)
    // Storage: Settlement UserAffirmations (r:0 w:220)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Asset BalanceOfAtScope (r:0 w:2)
    /// The range of component `f` is `[1, 10]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `o` is `[0, 10]`.
    fn execute_instruction_paused(f: u32, n: u32, o: u32) -> Weight {
        // Minimum execution time: 2_491_567 nanoseconds.
        Weight::from_ref_time(2_492_658_000)
            // Standard Error: 2_696_870
            .saturating_add(Weight::from_ref_time(107_976_366).saturating_mul(f.into()))
            // Standard Error: 260_401
            .saturating_add(Weight::from_ref_time(139_017_847).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().reads((23_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().reads((12_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().reads((4_u64).saturating_mul(o.into())))
            .saturating_add(DbWeight::get().writes(6))
            .saturating_add(DbWeight::get().writes((14_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().writes((11_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(o.into())))
    }
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Settlement InstructionAffirmsPending (r:1 w:1)
    // Storage: Settlement InstructionStatuses (r:1 w:1)
    // Storage: Settlement InstructionDetails (r:1 w:1)
    // Storage: Settlement InstructionLegs (r:121 w:120)
    // Storage: Settlement VenueFiltering (r:120 w:0)
    // Storage: Settlement InstructionMemos (r:1 w:0)
    // Storage: Settlement InstructionLegStatus (r:120 w:120)
    // Storage: Portfolio PortfolioLockedAssets (r:10 w:10)
    // Storage: Portfolio PortfolioLockedNFT (r:100 w:100)
    // Storage: Asset Frozen (r:110 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:10 w:0)
    // Storage: Portfolio Portfolios (r:20 w:0)
    // Storage: Asset Tokens (r:10 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:20 w:20)
    // Storage: Asset AggregateBalance (r:20 w:20)
    // Storage: Statistics AssetTransferCompliances (r:10 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity Claims (r:201 w:0)
    // Storage: Statistics AssetStats (r:140 w:100)
    // Storage: ComplianceManager AssetCompliances (r:110 w:0)
    // Storage: Asset BalanceOf (r:20 w:20)
    // Storage: Checkpoint Schedules (r:10 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:10 w:0)
    // Storage: Portfolio PortfolioAssetCount (r:10 w:10)
    // Storage: Statistics ActiveAssetStats (r:10 w:0)
    // Storage: NFT CollectionTicker (r:100 w:0)
    // Storage: NFT NumberOfNFTs (r:200 w:200)
    // Storage: Portfolio PortfolioNFT (r:100 w:200)
    // Storage: Settlement OffChainAffirmations (r:10 w:10)
    // Storage: Settlement AffirmsReceived (r:220 w:220)
    // Storage: Settlement UserAffirmations (r:0 w:220)
    // Storage: Settlement VenueInstructions (r:0 w:1)
    // Storage: Asset BalanceOfAtScope (r:0 w:2)
    /// The range of component `f` is `[1, 10]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `o` is `[0, 10]`.
    fn execute_scheduled_instruction(f: u32, n: u32, o: u32) -> Weight {
        // Minimum execution time: 4_865_627 nanoseconds.
        Weight::from_ref_time(4_868_046_000)
            // Standard Error: 4_968_761
            .saturating_add(Weight::from_ref_time(256_500_850).saturating_mul(f.into()))
            // Standard Error: 479_768
            .saturating_add(Weight::from_ref_time(132_285_533).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(8))
            .saturating_add(DbWeight::get().reads((57_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().reads((12_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().reads((4_u64).saturating_mul(o.into())))
            .saturating_add(DbWeight::get().writes(6))
            .saturating_add(DbWeight::get().writes((24_u64).saturating_mul(f.into())))
            .saturating_add(DbWeight::get().writes((11_u64).saturating_mul(n.into())))
            .saturating_add(DbWeight::get().writes((3_u64).saturating_mul(o.into())))
    }
    fn ensure_root_origin() -> Weight {
        // Minimum execution time: 900 nanoseconds.
        Weight::from_ref_time(910_000)
    }
}
