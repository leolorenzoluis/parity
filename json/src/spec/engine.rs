// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Engine deserialization.

use super::{Ethash, InstantSeal, BasicAuthority, AuthorityRound, Tendermint};

/// Engine deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub enum Engine {
	/// Null engine.
	#[serde(rename="null")]
	Null,
	/// Instantly sealing engine.
	#[serde(rename="instantSeal")]
	InstantSeal(InstantSeal),
	/// Ethash engine.
	Ethash(Ethash),
	/// BasicAuthority engine.
	#[serde(rename="basicAuthority")]
	BasicAuthority(BasicAuthority),
	/// AuthorityRound engine.
	#[serde(rename="authorityRound")]
	AuthorityRound(AuthorityRound),
	/// Tendermint engine.
	#[serde(rename="tendermint")]
	Tendermint(Tendermint)
}

#[cfg(test)]
mod tests {
	use serde_json;
	use spec::Engine;

	#[test]
	fn engine_deserialization() {
		let s = r#"{
			"null": null
		}"#;

		let deserialized: Engine = serde_json::from_str(s).unwrap();
		assert_eq!(Engine::Null, deserialized);

		let s = r#"{
			"instantSeal": { "params": {} }
		}"#;

		let deserialized: Engine = serde_json::from_str(s).unwrap();
		match deserialized {
			Engine::InstantSeal(_) => {},	// instant seal is unit tested in its own file.
			_ => assert!(false),
		};

		let s = r#"{
			"Ethash": {
				"params": {
					"gasLimitBoundDivisor": "0x0400",
					"minimumDifficulty": "0x020000",
					"difficultyBoundDivisor": "0x0800",
					"durationLimit": "0x0d",
					"blockReward": "0x4563918244F40000",
					"registrar" : "0xc6d9d2cd449a754c494264e1809c50e34d64562b",
					"homesteadTransition" : "0x",
					"daoHardforkTransition": "0xffffffffffffffff",
					"daoHardforkBeneficiary": "0x0000000000000000000000000000000000000000",
					"daoHardforkAccounts": []
				}
			}
		}"#;

		let deserialized: Engine = serde_json::from_str(s).unwrap();
		match deserialized {
			Engine::Ethash(_) => {},	// ethash is unit tested in its own file.
			_ => assert!(false),
		};

		let s = r#"{
			"basicAuthority": {
				"params": {
					"gasLimitBoundDivisor": "0x0400",
					"durationLimit": "0x0d",
					"validators" : {
						"list": ["0xc6d9d2cd449a754c494264e1809c50e34d64562b"]
					}
				}
			}
		}"#;
		let deserialized: Engine = serde_json::from_str(s).unwrap();
		match deserialized {
			Engine::BasicAuthority(_) => {}, // basicAuthority is unit tested in its own file.
			_ => assert!(false),
		};

		let s = r#"{
			"authorityRound": {
				"params": {
					"gasLimitBoundDivisor": "0x0400",
					"stepDuration": "0x02",
					"validators": {
						"list" : ["0xc6d9d2cd449a754c494264e1809c50e34d64562b"]
					},
					"blockReward": "0x50",
					"startStep" : 24,
					"eip155Transition": "0x42",
					"validateStepTransition": 150
				}
			}
		}"#;
		let deserialized: Engine = serde_json::from_str(s).unwrap();
		match deserialized {
			Engine::AuthorityRound(_) => {}, // AuthorityRound is unit tested in its own file.
			_ => assert!(false),
		};

		let s = r#"{
			"tendermint": {
				"params": {
					"gasLimitBoundDivisor": "0x0400",
					"validators": {
						"list": ["0xc6d9d2cd449a754c494264e1809c50e34d64562b"]
					},
					"blockReward": "0x50"
				}
			}
		}"#;
		let deserialized: Engine = serde_json::from_str(s).unwrap();
		match deserialized {
			Engine::Tendermint(_) => {}, // Tendermint is unit tested in its own file.
			_ => assert!(false),
		};
	}
}

