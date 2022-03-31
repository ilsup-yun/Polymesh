// This file is part of the Polymesh distribution (https://github.com/PolymathNetwork/Polymesh).
// Copyright (c) 2020 Polymath

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! # Contracts Module
//!
//! The contracts module provides the Layer 2 solution for Polymesh.
//! These smart contracts are defined using WASM,
//! and facilitated by the `ink!` DSL provided by Parity.
//!
//! With this module, you or Alice can instantiate such a contract,
//! through `instantiate_with_code` or `instantiate_with_hash`,
//! attaching its key as a secondary key of the signer's identity.
//! Anyone can then call this smart contract, e.g., Bob,
//! which may call back into the runtime to e.g., `create_asset`.
//! However, during the execution of `create_asset`,
//! the current identity will be Alice, as opposed to `Bob`.
//!
//! ## Overview
//!
//! The Contracts module provides functions for:
//!
//! - Instantiating contracts
//! - Calling contracts
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `instantiate_with_code` instantiates a contract with the code provided.
//! - `instantiate_with_hash` instantiates a contract by hash,
//!   assuming that a contract with the same code already was uploaded.
//! - `call` dispatches to the smart contract code, acting as the identity who made the contract.

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(associated_type_bounds)]

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

use codec::Decode;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::{
        DispatchError, DispatchErrorWithPostInfo, DispatchResultWithPostInfo, GetDispatchInfo,
    },
    ensure,
    traits::Get,
    weights::Weight,
};
use pallet_contracts::chain_extension as ce;
use pallet_contracts_primitives::{Code, ContractResult};
use pallet_identity::PermissionedCallOriginData;
use polymesh_common_utilities::traits::identity::Config as IdentityConfig;
use polymesh_common_utilities::{with_transaction, Context};
use polymesh_primitives::{Balance, Permissions};
use sp_core::crypto::UncheckedFrom;
use sp_core::Bytes;
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;

type Identity<T> = pallet_identity::Module<T>;
type BaseContracts<T> = pallet_contracts::Pallet<T>;
type CodeHash<T> = <T as frame_system::Config>::Hash;

pub trait WeightInfo {
    fn call() -> Weight;
    fn instantiate_with_code(code_len: u32, salt_len: u32) -> Weight;
    fn instantiate_with_hash(salt_len: u32) -> Weight;

    /// Computes the cost just for executing the chain extension,
    /// subtracting costs for `call` itself and runtime callbacks.
    fn chain_extension(in_len: u32) -> Weight {
        Self::chain_extension_full(in_len)
            .saturating_sub(Self::chain_extension_early_exit())
            .saturating_sub(Self::basic_runtime_call(in_len))
    }

    /// Returns the weight for a full execution of a smart contract `call` that
    /// calls  `register_custom_asset_type` in the runtime via a chain extension.
    /// The asset type is `in_len` characters long.
    fn chain_extension_full(in_len: u32) -> Weight;

    /// Returns the weight for a smart contract `call` that enters the chain extension
    /// but then immediately returns.
    fn chain_extension_early_exit() -> Weight;

    /// Returns the weight of executing `Asset::register_custom_asset_type`
    /// with an asset type that is `in_len` characters long.
    fn basic_runtime_call(in_len: u32) -> Weight;
}

/// The `Config` trait for the smart contracts pallet.
pub trait Config:
    IdentityConfig
    + pallet_contracts::Config<Currency = Self::Balances>
    + frame_system::Config<
        AccountId: AsRef<[u8]> + UncheckedFrom<<Self as frame_system::Config>::Hash>,
    >
{
    /// The overarching event type.
    type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;

    /// Max value that `in_len` can take, that is,
    /// the length of the data sent from a contract when making a runtime call.
    type MaxInLen: Get<u32>;

    /// The weight configuration for the pallet.
    type WeightInfo: WeightInfo;
}

decl_event! {
    pub enum Event {
        // This pallet does not directly define custom events.
        // See `pallet_contracts` and `pallet_identity`
        // for events currently emitted by extrinsics of this pallet.
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        /// The given `func_id: u32` did not translate into a known runtime call.
        RuntimeCallNotFound,
        /// Data left in input when decoding arguments of a call.
        DataLeftAfterDecoding,
        /// Input data that a contract passed when making a runtime call was too large.
        InLenTooLarge,
    }
}

decl_storage! {
    trait Store for Module<T: Config> as Contracts {
        // Storage items defined in `pallet_contracts` and `pallet_identity`.
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        fn on_initialize(block: T::BlockNumber) -> Weight {
            // Does he know what I do and... 🎶
            pallet_contracts::Pallet::<T>::on_initialize(block)
        }

        fn on_runtime_upgrade() -> Weight {
            // 🎶 ...You'll pass this on, won't you and?
            pallet_contracts::Pallet::<T>::on_runtime_upgrade()
        }

        /// Calls the `contract` through its address with the given `data`.
        ///
        /// The contract is endowed with `value` POLYX,
        /// but note that this is distinct from gas fees which are limited with `gas_limit`.
        ///
        /// The contract may optionally call back into the runtime,
        /// executing extrinsics such as e.g., `create_asset`.
        /// During such runtime calls, the current identity will be the one that instantiate the `contract`.
        /// This restriction exists for security purposes.
        ///
        /// # Arguments
        /// - `contract` to call.
        /// - `value` in POLYX to transfer to the contract.
        /// - `gas_limit` that limits how much gas execution can consume, erroring above it.
        /// - `data` to be interpreted by the smart contract call.
        ///
        /// # Errors
        /// - All the errors in `pallet_contracts::Call::call` can also happen here.
        /// - `ContractNotFound` if `contract` doesn't exist or isn't a contract.
        /// - CDD/Permissions are checked, unlike in `pallet_contracts`.
        #[weight = <T as Config>::WeightInfo::call().saturating_add(*gas_limit)]
        pub fn call(
            origin,
            contract: T::AccountId,
            value: Balance,
            gas_limit: Weight,
            data: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            Self::base_call(origin, contract, value, gas_limit, data)
        }

        /// Instantiates a smart contract defining it with the given `code` and `salt`.
        ///
        /// The contract will be attached as a secondary key,
        /// with `perms` as its permissions, to `origin`'s identity.
        ///
        /// The contract is transferred `endowment` amount of POLYX.
        /// This is distinct from the `gas_limit`,
        /// which controls how much gas the deployment code may at most consume.
        ///
        /// # Arguments
        /// - `endowment` amount of POLYX to transfer to the contract.
        /// - `gas_limit` for how much gas the `deploy` code in the contract may at most consume.
        /// - `code` with the WASM binary defining the smart contract.
        /// - `salt` used for contract address derivation.
        ///    By varying this, the same `code` can be used under the same identity.
        ///
        /// # Errors
        /// - All the errors in `pallet_contracts::Call::instantiate_with_code` can also happen here.
        /// - CDD/Permissions are checked, unlike in `pallet_contracts`.
        /// - Errors that arise when adding a new secondary key can also occur here.
        #[weight = <T as Config>::WeightInfo::instantiate_with_code(
            code.len() as u32,
            salt.len() as u32,
        )]
        pub fn instantiate_with_code(
            origin,
            endowment: Balance,
            gas_limit: Weight,
            code: Vec<u8>,
            data: Vec<u8>,
            salt: Vec<u8>,
            perms: Permissions
        ) -> DispatchResultWithPostInfo {
            Self::base_instantiate_with_code(origin, endowment, gas_limit, code, data, salt, perms)
        }

        /// Instantiates a smart contract defining using the given `code_hash` and `salt`.
        ///
        /// Unlike `instantiate_with_code`,
        /// this assumes that at least one contract with the same WASM code has already been uploaded.
        ///
        /// The contract will be attached as a secondary key,
        /// with `perms` as its permissions, to `origin`'s identity.
        ///
        /// The contract is transferred `endowment` amount of POLYX.
        /// This is distinct from the `gas_limit`,
        /// which controls how much gas the deployment code may at most consume.
        ///
        /// # Arguments
        /// - `endowment` amount of POLYX to transfer to the contract.
        /// - `gas_limit` for how much gas the `deploy` code in the contract may at most consume.
        /// - `code_hash` of an already uploaded WASM binary.
        /// - `salt` used for contract address derivation.
        ///    By varying this, the same `code` can be used under the same identity.
        ///
        /// # Errors
        /// - All the errors in `pallet_contracts::Call::instantiate` can also happen here.
        /// - CDD/Permissions are checked, unlike in `pallet_contracts`.
        /// - Errors that arise when adding a new secondary key can also occur here.
        #[weight = <T as Config>::WeightInfo::instantiate_with_hash(salt.len() as u32)]
        pub fn instantiate_with_hash(
            origin,
            endowment: Balance,
            gas_limit: Weight,
            code_hash: CodeHash<T>,
            data: Vec<u8>,
            salt: Vec<u8>,
            perms: Permissions
        ) -> DispatchResultWithPostInfo {
            Self::base_instantiate_with_hash(origin, endowment, gas_limit, code_hash, data, salt, perms)
        }
    }
}

impl<T: Config> Module<T> {
    /// Calls `contract` with `data`, gas limits, etc.
    /// The call is made as the DID the contract is a key of and not `origin`'s DID.
    fn base_call(
        origin: T::Origin,
        contract: T::AccountId,
        value: Balance,
        gas_limit: Weight,
        data: Vec<u8>,
    ) -> DispatchResultWithPostInfo {
        // Ensure contract caller has perms.
        let sender = Identity::<T>::ensure_origin_call_permissions(origin)?.sender;

        // Prevent pure `value` transfers when `contract` either doesn't exist or isn't a contract.
        ensure!(
            pallet_contracts::ContractInfoOf::<T>::contains_key(&contract),
            pallet_contracts::Error::<T>::ContractNotFound,
        );

        // Execute contract.
        Self::handle_error(
            <T as Config>::WeightInfo::call(),
            BaseContracts::<T>::bare_call(sender, contract, value, gas_limit, data, false),
        )
    }

    /// Instantiates a contract using `code` as the WASM code blob.
    fn base_instantiate_with_code(
        origin: T::Origin,
        endowment: Balance,
        gas_limit: Weight,
        code: Vec<u8>,
        inst_data: Vec<u8>,
        salt: Vec<u8>,
        perms: Permissions,
    ) -> DispatchResultWithPostInfo {
        Self::general_instantiate(
            origin,
            endowment,
            // Compute the base weight of roughly `base_instantiate`.
            <T as Config>::WeightInfo::instantiate_with_code(
                (code.len() / 1024) as u32,
                (salt.len() / 1024) as u32,
            ),
            gas_limit,
            T::Hashing::hash(&code),
            Code::Upload(Bytes(code)),
            inst_data,
            salt,
            perms,
        )
    }

    /// Instantiates a contract using an existing WASM code blob with `code_hash` as its code.
    fn base_instantiate_with_hash(
        origin: T::Origin,
        endowment: Balance,
        gas_limit: Weight,
        code_hash: CodeHash<T>,
        inst_data: Vec<u8>,
        salt: Vec<u8>,
        perms: Permissions,
    ) -> DispatchResultWithPostInfo {
        Self::general_instantiate(
            origin,
            endowment,
            // Compute the base weight of roughly `base_instantiate`.
            <T as Config>::WeightInfo::instantiate_with_hash((salt.len() / 1024) as u32),
            gas_limit,
            code_hash,
            Code::Existing(code_hash),
            inst_data,
            salt,
            perms,
        )
    }

    /// General logic for contract instantiation both when the code or code hash is given.
    ///
    /// The interesting parameters here are:
    /// - `base_weight` assigns a base non-variable weight to the full extrinsic.
    /// - `code_hash` specifies the hash to use to derive the contract's key.
    /// - `code` specifies the code for the contract, either as a code blob or an existing hash.
    ///   The hash of `code` in either case is assumed to correspond to `code_hash`.
    fn general_instantiate(
        origin: T::Origin,
        endowment: Balance,
        base_weight: Weight,
        gas_limit: Weight,
        code_hash: CodeHash<T>,
        code: Code<CodeHash<T>>,
        inst_data: Vec<u8>,
        salt: Vec<u8>,
        perms: Permissions,
    ) -> DispatchResultWithPostInfo {
        // Ensure we have perms + we'll need sender & DID.
        let PermissionedCallOriginData {
            primary_did: did,
            sender,
            ..
        } = Identity::<T>::ensure_origin_call_permissions(origin)?;

        // Pre-compute what contract's key will be...
        let contract_key = BaseContracts::<T>::contract_address(&sender, &code_hash, &salt);

        // ...and ensure that key can be a secondary-key of DID...
        Identity::<T>::ensure_secondary_key_can_be_added(&did, &contract_key, &perms)?;

        with_transaction(|| {
            // Roll back `join_identity` if contract was not instantiated.
            // ...so that the CDD due to `endowment` passes.
            Identity::<T>::unsafe_join_identity(did, perms, contract_key);

            // Now we can finally instantiate the contract.
            Self::handle_error(
                base_weight,
                BaseContracts::<T>::bare_instantiate(
                    sender.clone(),
                    endowment,
                    gas_limit,
                    code,
                    inst_data,
                    salt,
                    false,
                ),
            )
        })
    }

    /// Enriches `result` of executing a smart contract with actual weight,
    /// accounting for the consumed gas.
    fn handle_error<A>(
        base_weight: Weight,
        result: ContractResult<Result<A, DispatchError>>,
    ) -> DispatchResultWithPostInfo {
        let post_info = Some(result.gas_consumed.saturating_add(base_weight)).into();
        match result.result {
            Ok(_) => Ok(post_info),
            Err(error) => Err(DispatchErrorWithPostInfo { post_info, error }),
        }
    }
}

/// The `Call` enums of various pallets that the contracts pallet wants to know about.
pub enum CommonCall<T>
where
    T: Config + pallet_asset::Config,
{
    Asset(pallet_asset::Call<T>),
    Contracts(Call<T>),
}

/// An encoding of `func_id` into the encoding `0x_S_P_E_V`,
/// with each letter being a byte.
struct FuncId {
    /// Decides the version of the `extrinsic`.
    version: u8,
    /// Decides the `extrinsic` within the `pallet`.
    /// This isn't necessarily the same as the index within the pallet.
    extrinsic: u8,
    /// Decides the `pallet` within the runtime.
    /// This isn't necessarily the same as the index within the runtime.
    pallet: u8,
    /// Decides the scheme to use when interpreting `(extrinsic, pallet, version)`.
    scheme: u8,
}

/// Splits the `func_id` given from a smart contract into
/// the encoding `(scheme: u8, pallet: u8, extrinsic: u8, version: u8)`.
fn split_func_id(func_id: u32) -> FuncId {
    let extract = |which| (func_id >> which * 8) as u8;
    FuncId {
        version: extract(0),
        extrinsic: extract(1),
        pallet: extract(2),
        scheme: extract(3),
    }
}

/// Run `with` while the current DID is temporarily set to that of `key`.
fn with_key_as_current<T: Config, W: FnOnce() -> R, R>(key: &T::AccountId, with: W) -> R {
    let old_did = Context::current_identity::<Identity<T>>();
    let caller_did = Identity::<T>::key_to_identity_dids(key);
    Context::set_current_identity::<Identity<T>>(Some(caller_did));
    let result = with();
    Context::set_current_identity::<Identity<T>>(old_did);
    result
}

fn decode<V: Decode, T: Config>(input: &mut &[u8]) -> Result<V, DispatchError> {
    <_>::decode(input).map_err(|_| pallet_contracts::Error::<T>::DecodingFailed.into())
}

fn construct_call<T>(func_id: FuncId, input: &mut &[u8]) -> Result<CommonCall<T>, DispatchError>
where
    T: Config + pallet_asset::Config,
{
    /// Decode type from `input`.
    macro_rules! decode {
        () => {
            decode::<_, T>(input)?
        };
    }

    /// Pattern match on functions `0x00_pp_ee_00`.
    macro_rules! on {
        ($p:pat, $e:pat) => {
            FuncId {
                scheme: 0,
                pallet: $p,
                extrinsic: $e,
                version: 0,
            }
        };
    }

    Ok(match func_id {
        on!(0, 0) => CommonCall::Asset(pallet_asset::Call::register_ticker { ticker: decode!() }),
        on!(0, 1) => {
            CommonCall::Asset(pallet_asset::Call::accept_ticker_transfer { auth_id: decode!() })
        }
        on!(0, 2) => CommonCall::Asset(pallet_asset::Call::accept_asset_ownership_transfer {
            auth_id: decode!(),
        }),
        on!(0, 3) => CommonCall::Asset(pallet_asset::Call::create_asset {
            name: decode!(),
            ticker: decode!(),
            divisible: decode!(),
            asset_type: decode!(),
            identifiers: decode!(),
            funding_round: decode!(),
            disable_iu: decode!(),
        }),
        on!(0, 0x11) => {
            CommonCall::Asset(pallet_asset::Call::register_custom_asset_type { ty: decode!() })
        }
        _ => return Err(Error::<T>::RuntimeCallNotFound.into()),
    })
}

/// A chain extension allowing calls to polymesh pallets
/// and using the contract's DID instead of the caller's DID.
impl<T> ce::ChainExtension<T> for Module<T>
where
    <T as pallet_contracts::Config>::Call: From<CommonCall<T>> + GetDispatchInfo,
    T: Config + pallet_asset::Config,
{
    fn enabled() -> bool {
        true
    }

    fn call<E: ce::Ext<T = T>>(
        func_id: u32,
        env: ce::Environment<E, ce::InitState>,
    ) -> ce::Result<ce::RetVal> {
        // Used for benchmarking `chain_extension_early_exit`,
        // so we can remove the cost of the call + overhead of using any chain extension.
        // That is, we want to subtract costs not directly arising from *this* function body.
        // Caveat: This `if` imposes a minor cost during benchmarking but we'll live with that.
        #[cfg(feature = "runtime-benchmarks")]
        if func_id == 0 {
            return Ok(ce::RetVal::Converging(0));
        }

        let mut env = env.buf_in_buf_out();

        // Immediately charge weight as a linear function of `in_len`.
        let in_len = env.in_len();
        env.charge_weight(<T as Config>::WeightInfo::chain_extension(in_len))?;

        // Then limit `in_len` to a maximum.
        ensure!(
            in_len <= <T as Config>::MaxInLen::get(),
            Error::<T>::InLenTooLarge
        );

        // Decide what to call in the runtime.
        let func_id = split_func_id(func_id);
        let input = &mut &*env.read(in_len)?;
        let call: <T as pallet_contracts::Config>::Call =
            construct_call::<T>(func_id, input)?.into();
        ensure!(input.is_empty(), Error::<T>::DataLeftAfterDecoding);

        // Charge weight for the call.
        let di = call.get_dispatch_info();
        let charged_amount = env.charge_weight(di.weight)?;

        // Execute call requested by contract, with current DID set to the contract owner.
        let ext = env.ext();
        let result = with_key_as_current::<T, _, _>(ext.address(), || ext.call_runtime(call));

        // Refund unspent weight.
        let post_di = result.unwrap_or_else(|e| e.post_info);
        // This check isn't necessary but avoids some work.
        if post_di.actual_weight.is_some() {
            let actual_weight = post_di.calc_actual_weight(&di);
            env.adjust_weight(charged_amount, actual_weight);
        }

        // Ensure the call was successful.
        result.map_err(|e| e.error)?;

        // Done; continue with smart contract execution when returning.
        Ok(ce::RetVal::Converging(0))
    }
}