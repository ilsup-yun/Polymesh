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

//! Autogenerated weights for pallet_asset
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-17, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// -p=pallet_asset
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

/// Weights for pallet_asset using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_asset::WeightInfo for SubstrateWeight {
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Asset TickerConfig (r:1 w:0)
    // Storage: Asset Tickers (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Storage: Identity CurrentPayer (r:1 w:0)
    // Storage: Asset AssetOwnershipRelations (r:0 w:1)
    // Storage: Asset ClassicTickers (r:0 w:1)
    fn register_ticker() -> Weight {
        // Minimum execution time: 174_683 nanoseconds.
        Weight::from_ref_time(176_601_000)
            .saturating_add(DbWeight::get().reads(8))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity Authorizations (r:1 w:1)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Asset Tickers (r:1 w:1)
    // Storage: Identity AuthorizationsGiven (r:0 w:1)
    // Storage: Asset AssetOwnershipRelations (r:0 w:2)
    // Storage: Asset ClassicTickers (r:0 w:1)
    fn accept_ticker_transfer() -> Weight {
        // Minimum execution time: 125_908 nanoseconds.
        Weight::from_ref_time(137_524_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(6))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity Authorizations (r:1 w:1)
    // Storage: Asset Tokens (r:1 w:1)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset Tickers (r:1 w:1)
    // Storage: Identity AuthorizationsGiven (r:0 w:1)
    // Storage: Asset AssetOwnershipRelations (r:0 w:2)
    fn accept_asset_ownership_transfer() -> Weight {
        // Minimum execution time: 166_008 nanoseconds.
        Weight::from_ref_time(262_345_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(6))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Asset Tokens (r:1 w:1)
    // Storage: Asset TickerConfig (r:1 w:0)
    // Storage: Asset Tickers (r:1 w:1)
    // Storage: Identity DidRecords (r:1 w:1)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:2 w:0)
    // Storage: Identity CurrentPayer (r:1 w:0)
    // Storage: ExternalAgents NumFullAgents (r:1 w:1)
    // Storage: Asset FundingRound (r:0 w:1)
    // Storage: Asset AssetOwnershipRelations (r:0 w:1)
    // Storage: Asset AssetNames (r:0 w:1)
    // Storage: Asset ClassicTickers (r:0 w:1)
    // Storage: Asset DisableInvestorUniqueness (r:0 w:1)
    // Storage: Asset Identifiers (r:0 w:1)
    // Storage: ExternalAgents AgentOf (r:0 w:1)
    // Storage: ExternalAgents GroupOfAgent (r:0 w:1)
    /// The range of component `n` is `[1, 128]`.
    /// The range of component `i` is `[1, 512]`.
    /// The range of component `f` is `[1, 128]`.
    fn create_asset(n: u32, i: u32, f: u32) -> Weight {
        // Minimum execution time: 200_847 nanoseconds.
        Weight::from_ref_time(100_884_934)
            // Standard Error: 77_250
            .saturating_add(Weight::from_ref_time(278_828).saturating_mul(n.into()))
            // Standard Error: 19_270
            .saturating_add(Weight::from_ref_time(286_015).saturating_mul(i.into()))
            // Standard Error: 77_250
            .saturating_add(Weight::from_ref_time(634_940).saturating_mul(f.into()))
            .saturating_add(DbWeight::get().reads(11))
            .saturating_add(DbWeight::get().writes(12))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Asset Frozen (r:1 w:1)
    fn freeze() -> Weight {
        // Minimum execution time: 101_708 nanoseconds.
        Weight::from_ref_time(157_880_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Asset Frozen (r:1 w:1)
    fn unfreeze() -> Weight {
        // Minimum execution time: 96_056 nanoseconds.
        Weight::from_ref_time(101_288_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset AssetNames (r:0 w:1)
    /// The range of component `n` is `[1, 128]`.
    fn rename_asset(n: u32) -> Weight {
        // Minimum execution time: 89_390 nanoseconds.
        Weight::from_ref_time(109_928_382)
            // Standard Error: 37_735
            .saturating_add(Weight::from_ref_time(27_644).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Asset Tokens (r:1 w:1)
    // Storage: Asset BalanceOf (r:1 w:1)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:1)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Storage: Checkpoint Schedules (r:1 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Asset AggregateBalance (r:1 w:1)
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Storage: Asset FundingRound (r:1 w:0)
    // Storage: Asset IssuedInFundingRound (r:1 w:1)
    // Storage: Asset BalanceOfAtScope (r:0 w:1)
    fn issue() -> Weight {
        // Minimum execution time: 214_708 nanoseconds.
        Weight::from_ref_time(329_312_000)
            .saturating_add(DbWeight::get().reads(17))
            .saturating_add(DbWeight::get().writes(6))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Asset Tokens (r:1 w:1)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:1)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:0)
    // Storage: Asset BalanceOf (r:1 w:1)
    // Storage: Checkpoint Schedules (r:1 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Asset AggregateBalance (r:1 w:1)
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Storage: Asset BalanceOfAtScope (r:0 w:1)
    fn redeem() -> Weight {
        // Minimum execution time: 184_594 nanoseconds.
        Weight::from_ref_time(186_140_000)
            .saturating_add(DbWeight::get().reads(14))
            .saturating_add(DbWeight::get().writes(5))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset Tokens (r:1 w:1)
    fn make_divisible() -> Weight {
        // Minimum execution time: 88_774 nanoseconds.
        Weight::from_ref_time(90_687_000)
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset AssetDocumentsIdSequence (r:1 w:1)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Storage: Asset AssetDocuments (r:0 w:1)
    /// The range of component `d` is `[1, 64]`.
    fn add_documents(d: u32) -> Weight {
        // Minimum execution time: 116_522 nanoseconds.
        Weight::from_ref_time(119_051_008)
            // Standard Error: 366_370
            .saturating_add(Weight::from_ref_time(22_065_698).saturating_mul(d.into()))
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(1))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(d.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset AssetDocuments (r:0 w:1)
    /// The range of component `d` is `[1, 64]`.
    fn remove_documents(d: u32) -> Weight {
        // Minimum execution time: 68_210 nanoseconds.
        Weight::from_ref_time(89_335_411)
            // Standard Error: 231_300
            .saturating_add(Weight::from_ref_time(10_491_993).saturating_mul(d.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(d.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset FundingRound (r:0 w:1)
    /// The range of component `f` is `[1, 128]`.
    fn set_funding_round(_f: u32) -> Weight {
        // Minimum execution time: 82_368 nanoseconds.
        Weight::from_ref_time(106_047_388)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset Identifiers (r:0 w:1)
    /// The range of component `i` is `[1, 512]`.
    fn update_identifiers(i: u32) -> Weight {
        // Minimum execution time: 84_069 nanoseconds.
        Weight::from_ref_time(103_910_316)
            // Standard Error: 13_221
            .saturating_add(Weight::from_ref_time(129_898).saturating_mul(i.into()))
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Asset ClassicTickers (r:1 w:0)
    // Storage: Asset Tickers (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Asset AssetOwnershipRelations (r:0 w:2)
    fn claim_classic_ticker() -> Weight {
        // Minimum execution time: 171_335 nanoseconds.
        Weight::from_ref_time(172_705_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Asset Tickers (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Asset AssetOwnershipRelations (r:0 w:1)
    // Storage: Asset ClassicTickers (r:0 w:1)
    fn reserve_classic_ticker() -> Weight {
        // Minimum execution time: 81_020 nanoseconds.
        Weight::from_ref_time(82_208_000)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: Asset BalanceOf (r:2 w:2)
    // Storage: Checkpoint Schedules (r:1 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:2 w:2)
    // Storage: Portfolio PortfolioAssetCount (r:1 w:1)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Asset AggregateBalance (r:2 w:2)
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Storage: Asset BalanceOfAtScope (r:0 w:2)
    fn controller_transfer() -> Weight {
        // Minimum execution time: 238_657 nanoseconds.
        Weight::from_ref_time(250_330_000)
            .saturating_add(DbWeight::get().reads(17))
            .saturating_add(DbWeight::get().writes(9))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Asset CustomTypesInverse (r:1 w:1)
    // Storage: Asset CustomTypeIdSequence (r:1 w:1)
    // Storage: Asset CustomTypes (r:0 w:1)
    /// The range of component `n` is `[1, 2048]`.
    fn register_custom_asset_type(n: u32) -> Weight {
        // Minimum execution time: 70_349 nanoseconds.
        Weight::from_ref_time(87_312_169)
            // Standard Error: 2_459
            .saturating_add(Weight::from_ref_time(11_802).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset AssetMetadataGlobalKeyToName (r:1 w:0)
    // Storage: Asset AssetMetadataValueDetails (r:1 w:1)
    // Storage: Asset AssetMetadataValues (r:0 w:1)
    fn set_asset_metadata() -> Weight {
        // Minimum execution time: 120_764 nanoseconds.
        Weight::from_ref_time(136_224_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset AssetMetadataGlobalKeyToName (r:1 w:0)
    // Storage: Asset AssetMetadataValueDetails (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    fn set_asset_metadata_details() -> Weight {
        // Minimum execution time: 101_993 nanoseconds.
        Weight::from_ref_time(109_061_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset AssetMetadataLocalNameToKey (r:1 w:1)
    // Storage: Asset AssetMetadataNextLocalKey (r:1 w:1)
    // Storage: Asset AssetMetadataValueDetails (r:1 w:1)
    // Storage: Asset AssetMetadataValues (r:0 w:1)
    // Storage: Asset AssetMetadataLocalKeyToName (r:0 w:1)
    // Storage: Asset AssetMetadataLocalSpecs (r:0 w:1)
    fn register_and_set_local_asset_metadata() -> Weight {
        // Minimum execution time: 194_337 nanoseconds.
        Weight::from_ref_time(208_746_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(6))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset AssetMetadataLocalNameToKey (r:1 w:1)
    // Storage: Asset AssetMetadataNextLocalKey (r:1 w:1)
    // Storage: Asset AssetMetadataLocalKeyToName (r:0 w:1)
    // Storage: Asset AssetMetadataLocalSpecs (r:0 w:1)
    fn register_asset_metadata_local_type() -> Weight {
        // Minimum execution time: 142_589 nanoseconds.
        Weight::from_ref_time(162_084_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Asset AssetMetadataGlobalNameToKey (r:1 w:1)
    // Storage: Asset AssetMetadataNextGlobalKey (r:1 w:1)
    // Storage: Asset AssetMetadataGlobalKeyToName (r:0 w:1)
    // Storage: Asset AssetMetadataGlobalSpecs (r:0 w:1)
    fn register_asset_metadata_global_type() -> Weight {
        // Minimum execution time: 133_693 nanoseconds.
        Weight::from_ref_time(146_754_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Asset Tokens (r:1 w:1)
    // Storage: Portfolio Portfolios (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:1)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:0)
    // Storage: Portfolio PortfolioAssetCount (r:1 w:1)
    // Storage: Asset BalanceOf (r:1 w:1)
    // Storage: Checkpoint Schedules (r:1 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Asset AggregateBalance (r:1 w:1)
    // Storage: Statistics ActiveAssetStats (r:1 w:0)
    // Storage: Asset BalanceOfAtScope (r:0 w:1)
    fn redeem_from_portfolio() -> Weight {
        // Minimum execution time: 204_804 nanoseconds.
        Weight::from_ref_time(220_000_000)
            .saturating_add(DbWeight::get().reads(16))
            .saturating_add(DbWeight::get().writes(6))
    }
    // Storage: Asset Tokens (r:1 w:1)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    fn update_asset_type() -> Weight {
        // Minimum execution time: 109_111 nanoseconds.
        Weight::from_ref_time(161_990_000)
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset AssetMetadataLocalKeyToName (r:1 w:1)
    // Storage: Asset AssetMetadataValueDetails (r:1 w:1)
    // Storage: NFT CollectionTicker (r:1 w:0)
    // Storage: NFT CollectionKeys (r:1 w:0)
    // Storage: Asset AssetMetadataValues (r:0 w:1)
    // Storage: Asset AssetMetadataLocalNameToKey (r:0 w:1)
    // Storage: Asset AssetMetadataLocalSpecs (r:0 w:1)
    fn remove_local_metadata_key() -> Weight {
        // Minimum execution time: 129_153 nanoseconds.
        Weight::from_ref_time(137_340_000)
            .saturating_add(DbWeight::get().reads(8))
            .saturating_add(DbWeight::get().writes(5))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Asset AssetMetadataLocalKeyToName (r:1 w:0)
    // Storage: Asset AssetMetadataValueDetails (r:1 w:1)
    // Storage: Asset AssetMetadataValues (r:0 w:1)
    fn remove_metadata_value() -> Weight {
        // Minimum execution time: 103_155 nanoseconds.
        Weight::from_ref_time(136_947_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(2))
    }
}
