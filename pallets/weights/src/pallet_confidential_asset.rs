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

//! Autogenerated weights for pallet_confidential_asset
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-22, STEPS: `10`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512
//! HOSTNAME: `comp002`, CPU: `Intel(R) Xeon(R) CPU E5-2697 v2 @ 2.70GHz`

// Executed Command:
// target/release/polymesh
// benchmark
// pallet
// -p=pallet_confidential_asset
// -e=*
// -s
// 10
// -r
// 5
// --db-cache
// 512
// --heap-pages
// 4096
// --execution
// wasm
// --wasm-execution
// compiled
// --output
// ./
// --template
// ./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use polymesh_runtime_common::{RocksDbWeight as DbWeight, Weight};

/// Weights for pallet_confidential_asset using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_confidential_asset::WeightInfo for SubstrateWeight {
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset MercatAccountBalance (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountDid (r:1 w:1)
    fn validate_mercat_account() -> Weight {
        // Minimum execution time: 3_049_295 nanoseconds.
        Weight::from_ref_time(3_226_775_000)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset MediatorMercatAccounts (r:0 w:1)
    fn add_mediator_mercat_account() -> Weight {
        // Minimum execution time: 122_014 nanoseconds.
        Weight::from_ref_time(124_902_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset Details (r:1 w:1)
    fn create_confidential_asset() -> Weight {
        // Minimum execution time: 93_566 nanoseconds.
        Weight::from_ref_time(94_109_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset Details (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountBalance (r:1 w:1)
    fn mint_confidential_asset() -> Weight {
        // Minimum execution time: 5_815_000 nanoseconds.
        Weight::from_ref_time(5_893_129_000)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset MercatAccountDid (r:1 w:0)
    // Storage: ConfidentialAsset IncomingBalance (r:1 w:0)
    fn apply_incoming_balance() -> Weight {
        // Minimum execution time: 88_121 nanoseconds.
        Weight::from_ref_time(97_985_000).saturating_add(DbWeight::get().reads(3))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset VenueCounter (r:1 w:1)
    // Storage: ConfidentialAsset VenueCreator (r:0 w:1)
    // Storage: ConfidentialAsset IdentityVenues (r:0 w:1)
    fn create_venue() -> Weight {
        // Minimum execution time: 65_374 nanoseconds.
        Weight::from_ref_time(67_347_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset Details (r:1 w:0)
    // Storage: ConfidentialAsset VenueFiltering (r:0 w:1)
    fn set_venue_filtering() -> Weight {
        // Minimum execution time: 66_641 nanoseconds.
        Weight::from_ref_time(67_168_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset Details (r:1 w:0)
    // Storage: ConfidentialAsset VenueAllowList (r:0 w:11)
    /// The range of component `v` is `[0, 100]`.
    fn allow_venues(v: u32) -> Weight {
        // Minimum execution time: 63_080 nanoseconds.
        Weight::from_ref_time(66_246_933)
            // Standard Error: 80_891
            .saturating_add(Weight::from_ref_time(4_082_284).saturating_mul(v.into()))
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(v.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset Details (r:1 w:0)
    // Storage: ConfidentialAsset VenueAllowList (r:0 w:11)
    /// The range of component `v` is `[0, 100]`.
    fn disallow_venues(v: u32) -> Weight {
        // Minimum execution time: 63_408 nanoseconds.
        Weight::from_ref_time(62_533_500)
            // Standard Error: 87_832
            .saturating_add(Weight::from_ref_time(4_008_996).saturating_mul(v.into()))
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes((1_u64).saturating_mul(v.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset VenueCreator (r:1 w:0)
    // Storage: ConfidentialAsset TransactionCounter (r:1 w:1)
    // Storage: ConfidentialAsset VenueFiltering (r:1 w:0)
    // Storage: ConfidentialAsset VenueAllowList (r:1 w:0)
    // Storage: ConfidentialAsset MercatAccountDid (r:2 w:0)
    // Storage: ConfidentialAsset PendingAffirms (r:0 w:1)
    // Storage: ConfidentialAsset TransactionStatuses (r:0 w:1)
    // Storage: ConfidentialAsset UserAffirmations (r:0 w:3)
    // Storage: ConfidentialAsset TransactionLegs (r:0 w:1)
    // Storage: ConfidentialAsset VenueTransactions (r:0 w:1)
    // Storage: ConfidentialAsset Transactions (r:0 w:1)
    fn add_transaction() -> Weight {
        // Minimum execution time: 254_124 nanoseconds.
        Weight::from_ref_time(256_099_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(9))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionLegs (r:1 w:0)
    // Storage: ConfidentialAsset UserAffirmations (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountDid (r:1 w:0)
    // Storage: ConfidentialAsset MercatAccountBalance (r:1 w:1)
    // Storage: ConfidentialAsset RngNonce (r:1 w:1)
    // Storage: Babe NextRandomness (r:1 w:0)
    // Storage: Babe EpochStart (r:1 w:0)
    // Storage: ConfidentialAsset PendingAffirms (r:1 w:1)
    // Storage: ConfidentialAsset TxLegSenderAmount (r:0 w:1)
    // Storage: ConfidentialAsset SenderProofs (r:0 w:1)
    // Storage: ConfidentialAsset TxLegReceiverAmount (r:0 w:1)
    // Storage: ConfidentialAsset TxLegSenderBalance (r:0 w:1)
    fn sender_affirm_transaction() -> Weight {
        // Minimum execution time: 61_395_067 nanoseconds.
        Weight::from_ref_time(62_004_313_000)
            .saturating_add(DbWeight::get().reads(9))
            .saturating_add(DbWeight::get().writes(8))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionLegs (r:1 w:0)
    // Storage: ConfidentialAsset UserAffirmations (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountDid (r:1 w:0)
    // Storage: ConfidentialAsset PendingAffirms (r:1 w:1)
    fn receiver_affirm_transaction() -> Weight {
        // Minimum execution time: 115_340 nanoseconds.
        Weight::from_ref_time(119_175_000)
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionLegs (r:1 w:0)
    // Storage: ConfidentialAsset UserAffirmations (r:1 w:1)
    // Storage: ConfidentialAsset PendingAffirms (r:1 w:1)
    fn mediator_affirm_transaction() -> Weight {
        // Minimum execution time: 100_361 nanoseconds.
        Weight::from_ref_time(103_315_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset UserAffirmations (r:1 w:1)
    // Storage: ConfidentialAsset TransactionLegs (r:1 w:0)
    // Storage: ConfidentialAsset MercatAccountDid (r:1 w:0)
    // Storage: ConfidentialAsset TxLegSenderAmount (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountBalance (r:1 w:1)
    // Storage: ConfidentialAsset PendingAffirms (r:1 w:1)
    // Storage: ConfidentialAsset SenderProofs (r:0 w:1)
    // Storage: ConfidentialAsset TxLegReceiverAmount (r:0 w:1)
    // Storage: ConfidentialAsset TxLegSenderBalance (r:0 w:1)
    fn sender_unaffirm_transaction() -> Weight {
        // Minimum execution time: 393_277 nanoseconds.
        Weight::from_ref_time(425_489_000)
            .saturating_add(DbWeight::get().reads(7))
            .saturating_add(DbWeight::get().writes(7))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset UserAffirmations (r:1 w:1)
    // Storage: ConfidentialAsset TransactionLegs (r:1 w:0)
    // Storage: ConfidentialAsset MercatAccountDid (r:1 w:0)
    // Storage: ConfidentialAsset PendingAffirms (r:1 w:1)
    fn receiver_unaffirm_transaction() -> Weight {
        // Minimum execution time: 112_864 nanoseconds.
        Weight::from_ref_time(114_059_000)
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset UserAffirmations (r:1 w:1)
    // Storage: ConfidentialAsset TransactionLegs (r:1 w:0)
    // Storage: ConfidentialAsset PendingAffirms (r:1 w:1)
    fn mediator_unaffirm_transaction() -> Weight {
        // Minimum execution time: 123_911 nanoseconds.
        Weight::from_ref_time(129_220_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionLegs (r:2 w:1)
    // Storage: ConfidentialAsset PendingAffirms (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountDid (r:2 w:0)
    // Storage: ConfidentialAsset UserAffirmations (r:3 w:3)
    // Storage: ConfidentialAsset TxLegReceiverAmount (r:1 w:1)
    // Storage: ConfidentialAsset IncomingBalance (r:1 w:1)
    // Storage: ConfidentialAsset TransactionStatuses (r:0 w:1)
    // Storage: ConfidentialAsset TxLegSenderAmount (r:0 w:1)
    // Storage: ConfidentialAsset SenderProofs (r:0 w:1)
    // Storage: ConfidentialAsset Transactions (r:0 w:1)
    // Storage: ConfidentialAsset TxLegSenderBalance (r:0 w:1)
    /// The range of component `l` is `[1, 4]`.
    fn execute_transaction(l: u32) -> Weight {
        // Minimum execution time: 479_575 nanoseconds.
        Weight::from_ref_time(86_973_659)
            // Standard Error: 2_521_945
            .saturating_add(Weight::from_ref_time(423_527_881).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().reads((5_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((8_u64).saturating_mul(l.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionLegs (r:2 w:1)
    // Storage: ConfidentialAsset MercatAccountDid (r:2 w:0)
    // Storage: ConfidentialAsset UserAffirmations (r:1 w:3)
    // Storage: ConfidentialAsset TxLegSenderAmount (r:1 w:1)
    // Storage: ConfidentialAsset IncomingBalance (r:1 w:1)
    // Storage: ConfidentialAsset PendingAffirms (r:0 w:1)
    // Storage: ConfidentialAsset TransactionStatuses (r:0 w:1)
    // Storage: ConfidentialAsset SenderProofs (r:0 w:1)
    // Storage: ConfidentialAsset TxLegReceiverAmount (r:0 w:1)
    // Storage: ConfidentialAsset Transactions (r:0 w:1)
    // Storage: ConfidentialAsset TxLegSenderBalance (r:0 w:1)
    /// The range of component `l` is `[1, 4]`.
    fn revert_transaction(l: u32) -> Weight {
        // Minimum execution time: 453_220 nanoseconds.
        Weight::from_ref_time(54_785_125)
            // Standard Error: 2_253_282
            .saturating_add(Weight::from_ref_time(421_812_625).saturating_mul(l.into()))
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().reads((3_u64).saturating_mul(l.into())))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((8_u64).saturating_mul(l.into())))
    }
}