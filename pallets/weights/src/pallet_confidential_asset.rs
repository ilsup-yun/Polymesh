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
//! DATE: 2023-05-10, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512
//! HOSTNAME: `comp002`, CPU: `Intel(R) Xeon(R) CPU E5-2697 v2 @ 2.70GHz`

// Executed Command:
// target/release/polymesh
// benchmark
// pallet
// -p=pallet_confidential_asset
// -e=*
// -s
// 100
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
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset MercatAccountDid (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountBalance (r:1 w:1)
    fn validate_mercat_account() -> Weight {
        // Minimum execution time: 3_281_569 nanoseconds.
        Weight::from_ref_time(3_333_769_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset MediatorMercatAccounts (r:0 w:1)
    fn add_mediator_mercat_account() -> Weight {
        // Minimum execution time: 202_814 nanoseconds.
        Weight::from_ref_time(221_192_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset Details (r:1 w:1)
    fn create_confidential_asset() -> Weight {
        // Minimum execution time: 91_114 nanoseconds.
        Weight::from_ref_time(95_563_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset Details (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountBalance (r:1 w:1)
    fn mint_confidential_asset() -> Weight {
        // Minimum execution time: 5_742_498 nanoseconds.
        Weight::from_ref_time(5_820_015_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionCounter (r:1 w:1)
    // Storage: ConfidentialAsset TransactionLegs (r:0 w:1)
    fn add_transaction() -> Weight {
        // Minimum execution time: 335_492 nanoseconds.
        Weight::from_ref_time(336_354_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionLegs (r:1 w:0)
    // Storage: ConfidentialAsset TransactionProofs (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountBalance (r:1 w:0)
    // Storage: ConfidentialAsset PendingOutgoingBalance (r:1 w:1)
    // Storage: ConfidentialAsset IncomingBalance (r:1 w:0)
    // Storage: ConfidentialAsset FailedOutgoingBalance (r:1 w:0)
    // Storage: ConfidentialAsset RngNonce (r:1 w:1)
    // Storage: Babe NextRandomness (r:1 w:0)
    // Storage: Babe EpochStart (r:1 w:0)
    // Storage: ConfidentialAsset TxPendingState (r:0 w:1)
    fn sender_affirm_transaction() -> Weight {
        // Minimum execution time: 63_114_887 nanoseconds.
        Weight::from_ref_time(64_171_921_000)
            .saturating_add(DbWeight::get().reads(10))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionLegs (r:1 w:0)
    // Storage: ConfidentialAsset TransactionProofs (r:1 w:1)
    fn receiver_affirm_transaction() -> Weight {
        // Minimum execution time: 1_711_263 nanoseconds.
        Weight::from_ref_time(1_753_517_000)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionLegs (r:1 w:0)
    // Storage: ConfidentialAsset TransactionProofs (r:1 w:1)
    fn mediator_affirm_transaction() -> Weight {
        // Minimum execution time: 1_692_754 nanoseconds.
        Weight::from_ref_time(1_706_162_000)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset TransactionLegs (r:2 w:1)
    // Storage: ConfidentialAsset RngNonce (r:1 w:1)
    // Storage: Babe NextRandomness (r:1 w:0)
    // Storage: Babe EpochStart (r:1 w:0)
    // Storage: ConfidentialAsset TransactionProofs (r:1 w:1)
    // Storage: ConfidentialAsset TxPendingState (r:1 w:1)
    // Storage: ConfidentialAsset MercatAccountBalance (r:2 w:2)
    // Storage: ConfidentialAsset PendingOutgoingBalance (r:1 w:1)
    // Storage: ConfidentialAsset IncomingBalance (r:1 w:1)
    fn execute_transaction() -> Weight {
        // Minimum execution time: 62_259_539 nanoseconds.
        Weight::from_ref_time(62_459_787_000)
            .saturating_add(DbWeight::get().reads(12))
            .saturating_add(DbWeight::get().writes(8))
    }
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ConfidentialAsset IncomingBalance (r:1 w:0)
    // Storage: ConfidentialAsset FailedOutgoingBalance (r:1 w:0)
    // Storage: ConfidentialAsset MercatAccountBalance (r:1 w:0)
    fn reset_ordering_state() -> Weight {
        // Minimum execution time: 475_900 nanoseconds.
        Weight::from_ref_time(521_617_000).saturating_add(DbWeight::get().reads(5))
    }
}
