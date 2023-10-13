// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_statement
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-3-249`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/cord
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_statement
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/statement/src/weights.rs
// --header=./HEADER-GPL3
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_statement.
pub trait WeightInfo {
	fn create(l: u32, ) -> Weight;
	fn update(l: u32, ) -> Weight;
	fn revoke(l: u32, ) -> Weight;
	fn restore(l: u32, ) -> Weight;
	fn remove(l: u32, ) -> Weight;
	fn digest(l: u32, ) -> Weight;
}

/// Weights for pallet_statement using the CORD node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Registry Authorizations (r:1 w:0)
	/// Proof: Registry Authorizations (max_values: None, max_size: Some(203), added: 2678, mode: MaxEncodedLen)
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// Storage: Statement StatementDigests (r:0 w:1)
	/// Proof: Statement StatementDigests (max_values: None, max_size: Some(98), added: 2573, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn create(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `300`
		//  Estimated: `76533`
		// Minimum execution time: 31_519_000 picoseconds.
		Weight::from_parts(33_085_955, 76533)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// Storage: Statement StatementDigests (r:0 w:1)
	/// Proof: Statement StatementDigests (max_values: None, max_size: Some(98), added: 2573, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn update(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 23_778_000 picoseconds.
		Weight::from_parts(25_082_559, 76533)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn revoke(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 21_687_000 picoseconds.
		Weight::from_parts(22_799_814, 76533)
			// Standard Error: 8
			.saturating_add(Weight::from_parts(10, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn restore(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 21_594_000 picoseconds.
		Weight::from_parts(22_762_073, 76533)
			// Standard Error: 8
			.saturating_add(Weight::from_parts(4, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn remove(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 21_777_000 picoseconds.
		Weight::from_parts(23_046_073, 76533)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Statement Statements (r:1 w:0)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// Storage: Statement StatementDigests (r:0 w:1)
	/// Proof: Statement StatementDigests (max_values: None, max_size: Some(98), added: 2573, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn digest(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 21_831_000 picoseconds.
		Weight::from_parts(22_749_416, 76533)
			// Standard Error: 7
			.saturating_add(Weight::from_parts(32, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Registry Authorizations (r:1 w:0)
	/// Proof: Registry Authorizations (max_values: None, max_size: Some(203), added: 2678, mode: MaxEncodedLen)
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// Storage: Statement StatementDigests (r:0 w:1)
	/// Proof: Statement StatementDigests (max_values: None, max_size: Some(98), added: 2573, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn create(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `300`
		//  Estimated: `76533`
		// Minimum execution time: 31_519_000 picoseconds.
		Weight::from_parts(33_085_955, 76533)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// Storage: Statement StatementDigests (r:0 w:1)
	/// Proof: Statement StatementDigests (max_values: None, max_size: Some(98), added: 2573, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn update(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 23_778_000 picoseconds.
		Weight::from_parts(25_082_559, 76533)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn revoke(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 21_687_000 picoseconds.
		Weight::from_parts(22_799_814, 76533)
			// Standard Error: 8
			.saturating_add(Weight::from_parts(10, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn restore(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 21_594_000 picoseconds.
		Weight::from_parts(22_762_073, 76533)
			// Standard Error: 8
			.saturating_add(Weight::from_parts(4, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Statement Statements (r:1 w:1)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn remove(_l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 21_777_000 picoseconds.
		Weight::from_parts(23_046_073, 76533)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Statement Statements (r:1 w:0)
	/// Proof: Statement Statements (max_values: None, max_size: Some(232), added: 2707, mode: MaxEncodedLen)
	/// Storage: Statement Commits (r:1 w:1)
	/// Proof: Statement Commits (max_values: None, max_size: Some(73068), added: 75543, mode: MaxEncodedLen)
	/// Storage: Statement StatementDigests (r:0 w:1)
	/// Proof: Statement StatementDigests (max_values: None, max_size: Some(98), added: 2573, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 5120]`.
	fn digest(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `76533`
		// Minimum execution time: 21_831_000 picoseconds.
		Weight::from_parts(22_749_416, 76533)
			// Standard Error: 7
			.saturating_add(Weight::from_parts(32, 0).saturating_mul(l.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}