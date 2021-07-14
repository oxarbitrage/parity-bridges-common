// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Types used to connect to the Rialto-Substrate chain.

use codec::Encode;
use relay_substrate_client::{Chain, ChainBase, ChainWithBalances, TransactionSignScheme};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};
use std::time::Duration;

/// Rialto header id.
pub type HeaderId = relay_utils::HeaderId<rialto_runtime::Hash, rialto_runtime::BlockNumber>;

/// Rialto chain definition
#[derive(Debug, Clone, Copy)]
pub struct Rialto;

impl ChainBase for Rialto {
	type BlockNumber = rialto_runtime::BlockNumber;
	type Hash = rialto_runtime::Hash;
	type Hasher = rialto_runtime::Hashing;
	type Header = rialto_runtime::Header;
}

impl Chain for Rialto {
	const NAME: &'static str = "Rialto";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(5);
	const STORAGE_PROOF_OVERHEAD: u32 = bp_rialto::EXTRA_STORAGE_PROOF_SIZE;
	const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 = bp_rialto::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE;

	type AccountId = rialto_runtime::AccountId;
	type Index = rialto_runtime::Index;
	type SignedBlock = rialto_runtime::SignedBlock;
	type Call = rialto_runtime::Call;
	type Balance = rialto_runtime::Balance;
}

impl ChainWithBalances for Rialto {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		use frame_support::storage::generator::StorageMap;
		StorageKey(frame_system::Account::<rialto_runtime::Runtime>::storage_map_final_key(
			account_id,
		))
	}
}

impl TransactionSignScheme for Rialto {
	type Chain = Rialto;
	type AccountKeyPair = sp_core::sr25519::Pair;
	type SignedTransaction = rialto_runtime::UncheckedExtrinsic;

	fn sign_transaction(
		genesis_hash: <Self::Chain as ChainBase>::Hash,
		signer: &Self::AccountKeyPair,
		signer_nonce: <Self::Chain as Chain>::Index,
		call: <Self::Chain as Chain>::Call,
	) -> Self::SignedTransaction {
		let raw_payload = SignedPayload::from_raw(
			call,
			(
				frame_system::CheckSpecVersion::<rialto_runtime::Runtime>::new(),
				frame_system::CheckTxVersion::<rialto_runtime::Runtime>::new(),
				frame_system::CheckGenesis::<rialto_runtime::Runtime>::new(),
				frame_system::CheckEra::<rialto_runtime::Runtime>::from(sp_runtime::generic::Era::Immortal),
				frame_system::CheckNonce::<rialto_runtime::Runtime>::from(signer_nonce),
				frame_system::CheckWeight::<rialto_runtime::Runtime>::new(),
				pallet_transaction_payment::ChargeTransactionPayment::<rialto_runtime::Runtime>::from(0),
			),
			(
				rialto_runtime::VERSION.spec_version,
				rialto_runtime::VERSION.transaction_version,
				genesis_hash,
				genesis_hash,
				(),
				(),
				(),
			),
		);
		let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
		let signer: sp_runtime::MultiSigner = signer.public().into();
		let (call, extra, _) = raw_payload.deconstruct();

		rialto_runtime::UncheckedExtrinsic::new_signed(call, signer.into_account().into(), signature.into(), extra)
	}
}

/// Rialto signing params.
pub type SigningParams = sp_core::sr25519::Pair;

/// Rialto header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<rialto_runtime::Header>;
