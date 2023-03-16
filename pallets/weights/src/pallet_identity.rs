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

//! Autogenerated weights for pallet_identity
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
// -p=pallet_identity
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

/// Weights for pallet_identity using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_identity::WeightInfo for SubstrateWeight {
    // Storage: Identity KeyRecords (r:2 w:1)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Identity MultiPurposeNonce (r:1 w:1)
    // Storage: System ParentHash (r:1 w:0)
    // Storage: Identity DidRecords (r:1 w:1)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Storage: Identity DidKeys (r:0 w:1)
    // Storage: Identity AuthorizationsGiven (r:0 w:2)
    // Storage: Identity Authorizations (r:0 w:2)
    /// The range of component `i` is `[0, 200]`.
    fn cdd_register_did(i: u32) -> Weight {
        // Minimum execution time: 50_232 nanoseconds.
        Weight::from_ref_time(136_679_000)
            // Standard Error: 24_402
            .saturating_add(Weight::from_ref_time(20_897_000).saturating_mul(i.into()))
            .saturating_add(DbWeight::get().reads(8))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(i.into())))
            .saturating_add(DbWeight::get().writes(4))
            .saturating_add(DbWeight::get().writes((2_u64).saturating_mul(i.into())))
    }
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:1)
    // Storage: Instance2Group InactiveMembers (r:1 w:1)
    // Storage: Identity Claims (r:1 w:1)
    // Storage: Identity CurrentDid (r:1 w:0)
    fn invalidate_cdd_claims() -> Weight {
        // Minimum execution time: 58_468 nanoseconds.
        Weight::from_ref_time(102_834_000)
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity AccountKeyRefCount (r:2 w:0)
    // Storage: MultiSig MultiSigToIdentity (r:2 w:0)
    // Storage: Identity DidKeys (r:0 w:2)
    /// The range of component `i` is `[0, 200]`.
    fn remove_secondary_keys(i: u32) -> Weight {
        // Minimum execution time: 25_477 nanoseconds.
        Weight::from_ref_time(14_372_000)
            // Standard Error: 15_158
            .saturating_add(Weight::from_ref_time(25_538_000).saturating_mul(i.into()))
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().reads((3_u64).saturating_mul(i.into())))
            .saturating_add(DbWeight::get().writes((2_u64).saturating_mul(i.into())))
    }
    // Storage: Identity Authorizations (r:2 w:2)
    // Storage: Identity DidRecords (r:1 w:1)
    // Storage: Identity KeyRecords (r:2 w:2)
    // Storage: Identity AccountKeyRefCount (r:1 w:0)
    // Storage: MultiSig MultiSigToIdentity (r:1 w:0)
    // Storage: Identity CddAuthForPrimaryKeyRotation (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Identity AuthorizationsGiven (r:0 w:2)
    // Storage: Identity DidKeys (r:0 w:2)
    fn accept_primary_key() -> Weight {
        // Minimum execution time: 84_627 nanoseconds.
        Weight::from_ref_time(154_354_000)
            .saturating_add(DbWeight::get().reads(9))
            .saturating_add(DbWeight::get().writes(9))
    }
    // Storage: Identity Authorizations (r:2 w:2)
    // Storage: Identity DidRecords (r:1 w:1)
    // Storage: Identity KeyRecords (r:1 w:2)
    // Storage: Identity CddAuthForPrimaryKeyRotation (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Identity AuthorizationsGiven (r:0 w:2)
    // Storage: Identity DidKeys (r:0 w:1)
    fn rotate_primary_key_to_secondary() -> Weight {
        // Minimum execution time: 72_775 nanoseconds.
        Weight::from_ref_time(138_838_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(8))
    }
    // Storage: Identity CddAuthForPrimaryKeyRotation (r:0 w:1)
    fn change_cdd_requirement_for_mk_rotation() -> Weight {
        // Minimum execution time: 17_251 nanoseconds.
        Weight::from_ref_time(21_028_000).saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity Authorizations (r:1 w:1)
    // Storage: Identity DidRecords (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Instance2Group InactiveMembers (r:1 w:0)
    // Storage: Identity Claims (r:2 w:0)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Storage: Identity AuthorizationsGiven (r:0 w:1)
    // Storage: Identity DidKeys (r:0 w:1)
    // Storage: Identity CurrentDid (r:0 w:1)
    fn join_identity_as_key() -> Weight {
        // Minimum execution time: 71_953 nanoseconds.
        Weight::from_ref_time(130_869_000)
            .saturating_add(DbWeight::get().reads(10))
            .saturating_add(DbWeight::get().writes(5))
    }
    // Storage: Identity CurrentDid (r:1 w:0)
    // Storage: Identity KeyRecords (r:1 w:1)
    // Storage: Identity AccountKeyRefCount (r:1 w:0)
    // Storage: MultiSig MultiSigToIdentity (r:1 w:0)
    // Storage: Identity DidKeys (r:0 w:1)
    fn leave_identity_as_key() -> Weight {
        // Minimum execution time: 42_859 nanoseconds.
        Weight::from_ref_time(125_170_000)
            .saturating_add(DbWeight::get().reads(4))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity DidRecords (r:1 w:0)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity Claims (r:1 w:1)
    fn add_claim() -> Weight {
        // Minimum execution time: 47_197 nanoseconds.
        Weight::from_ref_time(99_733_000)
            .saturating_add(DbWeight::get().reads(6))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity Claims (r:1 w:1)
    // Storage: Asset BalanceOfAtScope (r:1 w:0)
    fn revoke_claim() -> Weight {
        // Minimum execution time: 44_692 nanoseconds.
        Weight::from_ref_time(77_210_000)
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity Claims (r:1 w:1)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Asset ScopeIdOf (r:1 w:0)
    // Storage: Asset BalanceOfAtScope (r:1 w:0)
    fn revoke_claim_by_index() -> Weight {
        // Minimum execution time: 48_860 nanoseconds.
        Weight::from_ref_time(95_452_000)
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:2 w:1)
    fn set_secondary_key_permissions() -> Weight {
        // Minimum execution time: 34_264 nanoseconds.
        Weight::from_ref_time(66_091_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(1))
    }
    /// The range of component `a` is `[0, 1000]`.
    /// The range of component `p` is `[0, 1000]`.
    /// The range of component `l` is `[0, 100]`.
    /// The range of component `e` is `[0, 100]`.
    fn permissions_cost(a: u32, p: u32, l: u32, e: u32) -> Weight {
        // Minimum execution time: 200_481 nanoseconds.
        Weight::from_ref_time(0)
            // Manually set for `a`
            .saturating_add(Weight::from_ref_time(163_000).saturating_mul(a.into()))
            // Manually set for `p`
            .saturating_add(Weight::from_ref_time(296_000).saturating_mul(p.into()))
            // Standard Error: 140_388
            .saturating_add(Weight::from_ref_time(33_906_000).saturating_mul(l.into()))
            // Standard Error: 140_388
            .saturating_add(Weight::from_ref_time(33_316_000).saturating_mul(e.into()))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity IsDidFrozen (r:0 w:1)
    fn freeze_secondary_keys() -> Weight {
        // Minimum execution time: 26_959 nanoseconds.
        Weight::from_ref_time(41_786_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity IsDidFrozen (r:0 w:1)
    fn unfreeze_secondary_keys() -> Weight {
        // Minimum execution time: 27_130 nanoseconds.
        Weight::from_ref_time(38_602_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity MultiPurposeNonce (r:1 w:1)
    // Storage: Identity AuthorizationsGiven (r:0 w:1)
    // Storage: Identity Authorizations (r:0 w:1)
    fn add_authorization() -> Weight {
        // Minimum execution time: 33_202 nanoseconds.
        Weight::from_ref_time(96_963_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(3))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity Authorizations (r:1 w:1)
    // Storage: Identity AuthorizationsGiven (r:0 w:1)
    fn remove_authorization() -> Weight {
        // Minimum execution time: 35_346 nanoseconds.
        Weight::from_ref_time(79_037_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Identity OffChainAuthorizationNonce (r:1 w:1)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    // Storage: Identity DidKeys (r:0 w:2)
    /// The range of component `i` is `[0, 200]`.
    fn add_secondary_keys_with_authorization(i: u32) -> Weight {
        // Minimum execution time: 37_910 nanoseconds.
        Weight::from_ref_time(279_659_000)
            // Standard Error: 62_588
            .saturating_add(Weight::from_ref_time(96_044_000).saturating_mul(i.into()))
            .saturating_add(DbWeight::get().reads(5))
            .saturating_add(DbWeight::get().reads((1_u64).saturating_mul(i.into())))
            .saturating_add(DbWeight::get().writes(1))
            .saturating_add(DbWeight::get().writes((2_u64).saturating_mul(i.into())))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity DidRecords (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Instance2Group InactiveMembers (r:1 w:0)
    // Storage: Identity Claims (r:3 w:1)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Asset ScopeIdOf (r:1 w:1)
    // Storage: Asset BalanceOfAtScope (r:1 w:1)
    // Storage: Asset BalanceOf (r:1 w:0)
    // Storage: Asset AggregateBalance (r:1 w:1)
    fn add_investor_uniqueness_claim() -> Weight {
        // Minimum execution time: 1_009_724 nanoseconds.
        Weight::from_ref_time(2_373_112_000)
            .saturating_add(DbWeight::get().reads(13))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity DidRecords (r:1 w:0)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Instance2Group ActiveMembers (r:1 w:0)
    // Storage: Instance2Group InactiveMembers (r:1 w:0)
    // Storage: Identity Claims (r:3 w:1)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Asset ScopeIdOf (r:1 w:1)
    // Storage: Asset BalanceOfAtScope (r:1 w:1)
    // Storage: Asset BalanceOf (r:1 w:0)
    // Storage: Asset AggregateBalance (r:1 w:1)
    fn add_investor_uniqueness_claim_v2() -> Weight {
        // Minimum execution time: 1_913_584 nanoseconds.
        Weight::from_ref_time(4_451_186_000)
            .saturating_add(DbWeight::get().reads(13))
            .saturating_add(DbWeight::get().writes(4))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity CustomClaimsInverse (r:1 w:1)
    // Storage: Identity CustomClaimIdSequence (r:1 w:1)
    // Storage: Identity CustomClaims (r:0 w:1)
    /// The range of component `n` is `[1, 2048]`.
    fn register_custom_claim_type(n: u32) -> Weight {
        // Minimum execution time: 34_513 nanoseconds.
        Weight::from_ref_time(60_556_000)
            // Standard Error: 234
            .saturating_add(Weight::from_ref_time(14_000).saturating_mul(n.into()))
            .saturating_add(DbWeight::get().reads(3))
            .saturating_add(DbWeight::get().writes(3))
    }
}
