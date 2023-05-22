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

//! Autogenerated weights for pallet_indices
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
// -p=pallet_indices
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

/// Weights for pallet_indices using the Substrate node and recommended hardware.
pub struct SubstrateWeight;
impl pallet_indices::WeightInfo for SubstrateWeight {
    // Storage: Indices Accounts (r:1 w:1)
    fn claim() -> Weight {
        // Minimum execution time: 114_567 nanoseconds.
        Weight::from_ref_time(120_141_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Indices Accounts (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn transfer() -> Weight {
        // Minimum execution time: 133_213 nanoseconds.
        Weight::from_ref_time(224_234_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Indices Accounts (r:1 w:1)
    fn free() -> Weight {
        // Minimum execution time: 68_430 nanoseconds.
        Weight::from_ref_time(105_571_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
    // Storage: Indices Accounts (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn force_transfer() -> Weight {
        // Minimum execution time: 163_948 nanoseconds.
        Weight::from_ref_time(175_522_000)
            .saturating_add(DbWeight::get().reads(2))
            .saturating_add(DbWeight::get().writes(2))
    }
    // Storage: Indices Accounts (r:1 w:1)
    fn freeze() -> Weight {
        // Minimum execution time: 69_969 nanoseconds.
        Weight::from_ref_time(71_946_000)
            .saturating_add(DbWeight::get().reads(1))
            .saturating_add(DbWeight::get().writes(1))
    }
}
