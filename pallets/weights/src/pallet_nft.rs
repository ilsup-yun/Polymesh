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

//! Autogenerated weights for pallet_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-28, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512

// Executed Command:
// ./target/release/polymesh
// benchmark
// pallet
// -s
// 100
// -r
// 5
// -p=pallet_nft
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

/// Weights for pallet_nft using the Substrate node and recommended hardware.
pub struct WeightInfo;
impl pallet_nft::WeightInfo for WeightInfo {
    // Storage: Asset Tokens (r:1 w:1)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: NFT CollectionTicker (r:1 w:1)
    // Storage: Asset AssetMetadataGlobalKeyToName (r:1 w:0)
    // Storage: Asset TickerConfig (r:1 w:0)
    // Storage: Asset Tickers (r:1 w:1)
    // Storage: Identity DidRecords (r:1 w:1)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:2 w:0)
    // Storage: Identity CurrentPayer (r:1 w:0)
    // Storage: ExternalAgents NumFullAgents (r:1 w:1)
    // Storage: NFT NextCollectionId (r:1 w:1)
    // Storage: NFT Collection (r:0 w:1)
    // Storage: NFT CollectionKeys (r:0 w:1)
    // Storage: Asset FundingRound (r:0 w:1)
    // Storage: Asset AssetOwnershipRelations (r:0 w:1)
    // Storage: Asset AssetNames (r:0 w:1)
    // Storage: Asset ClassicTickers (r:0 w:1)
    // Storage: Asset DisableInvestorUniqueness (r:0 w:1)
    // Storage: Asset Identifiers (r:0 w:1)
    // Storage: ExternalAgents AgentOf (r:0 w:1)
    // Storage: ExternalAgents GroupOfAgent (r:0 w:1)
    fn create_nft_collection(n: u32) -> Weight {
        (150_374_000 as Weight)
            // Standard Error: 16_000
            .saturating_add((4_253_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(DbWeight::get().reads(13 as Weight))
            .saturating_add(DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
            .saturating_add(DbWeight::get().writes(16 as Weight))
    }
    // Storage: NFT CollectionTicker (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: NFT CollectionKeys (r:1 w:0)
    // Storage: NFT NumberOfNFTs (r:1 w:1)
    // Storage: NFT NextNFTId (r:1 w:1)
    // Storage: Portfolio PortfolioNFT (r:0 w:1)
    // Storage: NFT MetadataValue (r:0 w:1)
    fn issue_nft(n: u32) -> Weight {
        (82_284_000 as Weight)
            // Standard Error: 14_000
            .saturating_add((3_072_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(DbWeight::get().reads(9 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
    }
    // Storage: NFT CollectionTicker (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Portfolio PortfolioNFT (r:1 w:1)
    // Storage: NFT NumberOfNFTs (r:1 w:1)
    // Storage: NFT MetadataValue (r:0 w:1)
    fn redeem_nft(n: u32) -> Weight {
        (87_585_000 as Weight)
            // Standard Error: 7_000
            .saturating_add((1_021_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(DbWeight::get().reads(8 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
            .saturating_add(DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
    }
}