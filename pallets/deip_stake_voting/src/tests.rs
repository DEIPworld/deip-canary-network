// Tests for StakeVoting Pallet

#![cfg(test)]

use super::*;

use crate as pallet_multisig;
use frame_support::{
	assert_noop, assert_ok, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, Contains},
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type OpaqueCall = super::OpaqueCall<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Assets: pallet_deip_assets::{Pallet, Call, Storage, Config<T>, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        DeipUniques: pallet_deip_uniques::{Pallet, Storage, Config<T>},
		StakeVoting: pallet_deip_stake_voting::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
	type BaseCallFilter = TestBaseCallFilter;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Call = Call;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

pub struct TestBaseCallFilter;
impl Contains<Call> for TestBaseCallFilter {
	fn contains(c: &Call) -> bool {
		match *c {
			Call::Balances(_) => true,
			// Needed for benchmarking
			Call::System(frame_system::Call::remark { .. }) => true,
			_ => false,
		}
	}
}
impl Config for Test {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
    type Assets = Assets;
    type AssetId = u32;
    type AssetBalance = u64;
	type DepositBase = ConstU64<1>;
	type RelativeThresholdLimit = ConstU64<100>;
	type WeightInfo = ();
}

use pallet_balances::{Call as BalancesCall, Error as BalancesError};

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10), (2, 10), (3, 10), (4, 10), (5, 2)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	pallet_assets::GenesisConfig::<Test> {
        assets: vec![(1, 1, true, 10000)],
        metadata: vec![(1, vec![2], vec![3], 4)],
        accounts: vec![
            (1, 1, 4000),
            (1, 2, 2000),
            (1, 3, 2000),
            (1, 4, 1000),
            (1, 5, 1000),
        ]
	}
	.assimilate_storage(&mut t)
	.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

fn now() -> Timepoint<u64> {
	StakeVoting::timepoint()
}

fn call_transfer(dest: u64, value: u64) -> Call {
	Call::Balances(BalancesCall::transfer { dest, value })
}

/*#[test]
fn multisig_deposit_is_taken_and_returned() {
	new_test_ext().execute_with(|| {
		let multi = StakeVoting::voting_account_id(&[1, 2, 3][..], 2);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15);
		let call_weight = call.get_dispatch_info().weight;
		let data = call.encode();
		assert_ok!(StakeVoting::as_multi(
			Origin::signed(1),
			2,
			vec![2, 3],
			None,
			OpaqueCall::from_encoded(data.clone()),
			false,
			0
		));
		assert_eq!(Balances::free_balance(1), 2);
		assert_eq!(Balances::reserved_balance(1), 3);

		assert_ok!(StakeVoting::as_multi(
			Origin::signed(2),
			2,
			vec![1, 3],
			Some(now()),
			OpaqueCall::from_encoded(data),
			false,
			call_weight
		));
		assert_eq!(Balances::free_balance(1), 5);
		assert_eq!(Balances::reserved_balance(1), 0);
	});
}

#[test]
fn multisig_deposit_is_taken_and_returned_with_call_storage() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 2);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15);
		let call_weight = call.get_dispatch_info().weight;
		let data = call.encode();
		let hash = blake2_256(&data);
		assert_ok!(Multisig::as_multi(
			Origin::signed(1),
			2,
			vec![2, 3],
			None,
			OpaqueCall::from_encoded(data),
			true,
			0
		));
		assert_eq!(Balances::free_balance(1), 0);
		assert_eq!(Balances::reserved_balance(1), 5);

		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(2),
			2,
			vec![1, 3],
			Some(now()),
			hash,
			call_weight
		));
		assert_eq!(Balances::free_balance(1), 5);
		assert_eq!(Balances::reserved_balance(1), 0);
	});
}

#[test]
fn multisig_deposit_is_taken_and_returned_with_alt_call_storage() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 3);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15);
		let call_weight = call.get_dispatch_info().weight;
		let data = call.encode();
		let hash = blake2_256(&data);

		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			None,
			hash.clone(),
			0
		));
		assert_eq!(Balances::free_balance(1), 1);
		assert_eq!(Balances::reserved_balance(1), 4);

		assert_ok!(Multisig::as_multi(
			Origin::signed(2),
			3,
			vec![1, 3],
			Some(now()),
			OpaqueCall::from_encoded(data),
			true,
			0
		));
		assert_eq!(Balances::free_balance(2), 3);
		assert_eq!(Balances::reserved_balance(2), 2);
		assert_eq!(Balances::free_balance(1), 1);
		assert_eq!(Balances::reserved_balance(1), 4);

		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(3),
			3,
			vec![1, 2],
			Some(now()),
			hash,
			call_weight
		));
		assert_eq!(Balances::free_balance(1), 5);
		assert_eq!(Balances::reserved_balance(1), 0);
		assert_eq!(Balances::free_balance(2), 5);
		assert_eq!(Balances::reserved_balance(2), 0);
	});
}

#[test]
fn cancel_multisig_returns_deposit() {
	new_test_ext().execute_with(|| {
		let call = call_transfer(6, 15).encode();
		let hash = blake2_256(&call);
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			None,
			hash.clone(),
			0
		));
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(2),
			3,
			vec![1, 3],
			Some(now()),
			hash.clone(),
			0
		));
		assert_eq!(Balances::free_balance(1), 6);
		assert_eq!(Balances::reserved_balance(1), 4);
		assert_ok!(Multisig::cancel_as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			now(),
			hash.clone()
		),);
		assert_eq!(Balances::free_balance(1), 10);
		assert_eq!(Balances::reserved_balance(1), 0);
	});
}

#[test]
fn timepoint_checking_works() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 2);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15).encode();
		let hash = blake2_256(&call);

		assert_noop!(
			Multisig::approve_as_multi(
				Origin::signed(2),
				2,
				vec![1, 3],
				Some(now()),
				hash.clone(),
				0
			),
			Error::<Test>::UnexpectedTimepoint,
		);

		assert_ok!(Multisig::approve_as_multi(Origin::signed(1), 2, vec![2, 3], None, hash, 0));

		assert_noop!(
			Multisig::as_multi(
				Origin::signed(2),
				2,
				vec![1, 3],
				None,
				OpaqueCall::from_encoded(call.clone()),
				false,
				0
			),
			Error::<Test>::NoTimepoint,
		);
		let later = Timepoint { index: 1, ..now() };
		assert_noop!(
			Multisig::as_multi(
				Origin::signed(2),
				2,
				vec![1, 3],
				Some(later),
				OpaqueCall::from_encoded(call),
				false,
				0
			),
			Error::<Test>::WrongTimepoint,
		);
	});
}

#[test]
fn multisig_2_of_3_works_with_call_storing() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 2);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15);
		let call_weight = call.get_dispatch_info().weight;
		let data = call.encode();
		let hash = blake2_256(&data);
		assert_ok!(Multisig::as_multi(
			Origin::signed(1),
			2,
			vec![2, 3],
			None,
			OpaqueCall::from_encoded(data),
			true,
			0
		));
		assert_eq!(Balances::free_balance(6), 0);

		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(2),
			2,
			vec![1, 3],
			Some(now()),
			hash,
			call_weight
		));
		assert_eq!(Balances::free_balance(6), 15);
	});
}

#[test]
fn multisig_2_of_3_works() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 2);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15);
		let call_weight = call.get_dispatch_info().weight;
		let data = call.encode();
		let hash = blake2_256(&data);
		assert_ok!(Multisig::approve_as_multi(Origin::signed(1), 2, vec![2, 3], None, hash, 0));
		assert_eq!(Balances::free_balance(6), 0);

		assert_ok!(Multisig::as_multi(
			Origin::signed(2),
			2,
			vec![1, 3],
			Some(now()),
			OpaqueCall::from_encoded(data),
			false,
			call_weight
		));
		assert_eq!(Balances::free_balance(6), 15);
	});
}

#[test]
fn multisig_3_of_3_works() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 3);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15);
		let call_weight = call.get_dispatch_info().weight;
		let data = call.encode();
		let hash = blake2_256(&data);
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			None,
			hash.clone(),
			0
		));
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(2),
			3,
			vec![1, 3],
			Some(now()),
			hash.clone(),
			0
		));
		assert_eq!(Balances::free_balance(6), 0);

		assert_ok!(Multisig::as_multi(
			Origin::signed(3),
			3,
			vec![1, 2],
			Some(now()),
			OpaqueCall::from_encoded(data),
			false,
			call_weight
		));
		assert_eq!(Balances::free_balance(6), 15);
	});
}

#[test]
fn cancel_multisig_works() {
	new_test_ext().execute_with(|| {
		let call = call_transfer(6, 15).encode();
		let hash = blake2_256(&call);
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			None,
			hash.clone(),
			0
		));
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(2),
			3,
			vec![1, 3],
			Some(now()),
			hash.clone(),
			0
		));
		assert_noop!(
			Multisig::cancel_as_multi(Origin::signed(2), 3, vec![1, 3], now(), hash.clone()),
			Error::<Test>::NotOwner,
		);
		assert_ok!(Multisig::cancel_as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			now(),
			hash.clone()
		),);
	});
}

#[test]
fn cancel_multisig_with_call_storage_works() {
	new_test_ext().execute_with(|| {
		let call = call_transfer(6, 15).encode();
		let hash = blake2_256(&call);
		assert_ok!(Multisig::as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			None,
			OpaqueCall::from_encoded(call),
			true,
			0
		));
		assert_eq!(Balances::free_balance(1), 4);
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(2),
			3,
			vec![1, 3],
			Some(now()),
			hash.clone(),
			0
		));
		assert_noop!(
			Multisig::cancel_as_multi(Origin::signed(2), 3, vec![1, 3], now(), hash.clone()),
			Error::<Test>::NotOwner,
		);
		assert_ok!(Multisig::cancel_as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			now(),
			hash.clone()
		),);
		assert_eq!(Balances::free_balance(1), 10);
	});
}

#[test]
fn cancel_multisig_with_alt_call_storage_works() {
	new_test_ext().execute_with(|| {
		let call = call_transfer(6, 15).encode();
		let hash = blake2_256(&call);
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			None,
			hash.clone(),
			0
		));
		assert_eq!(Balances::free_balance(1), 6);
		assert_ok!(Multisig::as_multi(
			Origin::signed(2),
			3,
			vec![1, 3],
			Some(now()),
			OpaqueCall::from_encoded(call),
			true,
			0
		));
		assert_eq!(Balances::free_balance(2), 8);
		assert_ok!(Multisig::cancel_as_multi(Origin::signed(1), 3, vec![2, 3], now(), hash));
		assert_eq!(Balances::free_balance(1), 10);
		assert_eq!(Balances::free_balance(2), 10);
	});
}

#[test]
fn multisig_2_of_3_as_multi_works() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 2);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15);
		let call_weight = call.get_dispatch_info().weight;
		let data = call.encode();
		assert_ok!(Multisig::as_multi(
			Origin::signed(1),
			2,
			vec![2, 3],
			None,
			OpaqueCall::from_encoded(data.clone()),
			false,
			0
		));
		assert_eq!(Balances::free_balance(6), 0);

		assert_ok!(Multisig::as_multi(
			Origin::signed(2),
			2,
			vec![1, 3],
			Some(now()),
			OpaqueCall::from_encoded(data),
			false,
			call_weight
		));
		assert_eq!(Balances::free_balance(6), 15);
	});
}

#[test]
fn multisig_2_of_3_as_multi_with_many_calls_works() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 2);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call1 = call_transfer(6, 10);
		let call1_weight = call1.get_dispatch_info().weight;
		let data1 = call1.encode();
		let call2 = call_transfer(7, 5);
		let call2_weight = call2.get_dispatch_info().weight;
		let data2 = call2.encode();

		assert_ok!(Multisig::as_multi(
			Origin::signed(1),
			2,
			vec![2, 3],
			None,
			OpaqueCall::from_encoded(data1.clone()),
			false,
			0
		));
		assert_ok!(Multisig::as_multi(
			Origin::signed(2),
			2,
			vec![1, 3],
			None,
			OpaqueCall::from_encoded(data2.clone()),
			false,
			0
		));
		assert_ok!(Multisig::as_multi(
			Origin::signed(3),
			2,
			vec![1, 2],
			Some(now()),
			OpaqueCall::from_encoded(data1),
			false,
			call1_weight
		));
		assert_ok!(Multisig::as_multi(
			Origin::signed(3),
			2,
			vec![1, 2],
			Some(now()),
			OpaqueCall::from_encoded(data2),
			false,
			call2_weight
		));

		assert_eq!(Balances::free_balance(6), 10);
		assert_eq!(Balances::free_balance(7), 5);
	});
}

#[test]
fn multisig_2_of_3_cannot_reissue_same_call() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 2);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 10);
		let call_weight = call.get_dispatch_info().weight;
		let data = call.encode();
		let hash = blake2_256(&data);
		assert_ok!(Multisig::as_multi(
			Origin::signed(1),
			2,
			vec![2, 3],
			None,
			OpaqueCall::from_encoded(data.clone()),
			false,
			0
		));
		assert_ok!(Multisig::as_multi(
			Origin::signed(2),
			2,
			vec![1, 3],
			Some(now()),
			OpaqueCall::from_encoded(data.clone()),
			false,
			call_weight
		));
		assert_eq!(Balances::free_balance(multi), 5);

		assert_ok!(Multisig::as_multi(
			Origin::signed(1),
			2,
			vec![2, 3],
			None,
			OpaqueCall::from_encoded(data.clone()),
			false,
			0
		));
		assert_ok!(Multisig::as_multi(
			Origin::signed(3),
			2,
			vec![1, 2],
			Some(now()),
			OpaqueCall::from_encoded(data),
			false,
			call_weight
		));

		let err = DispatchError::from(BalancesError::<Test, _>::InsufficientBalance).stripped();
		System::assert_last_event(
			pallet_multisig::Event::MultisigExecuted {
				approving: 3,
				timepoint: now(),
				multisig: multi,
				call_hash: hash,
				result: Err(err),
			}
			.into(),
		);
	});
}

#[test]
fn minimum_threshold_check_works() {
	new_test_ext().execute_with(|| {
		let call = call_transfer(6, 15).encode();
		assert_noop!(
			Multisig::as_multi(
				Origin::signed(1),
				0,
				vec![2],
				None,
				OpaqueCall::from_encoded(call.clone()),
				false,
				0
			),
			Error::<Test>::MinimumThreshold,
		);
		assert_noop!(
			Multisig::as_multi(
				Origin::signed(1),
				1,
				vec![2],
				None,
				OpaqueCall::from_encoded(call.clone()),
				false,
				0
			),
			Error::<Test>::MinimumThreshold,
		);
	});
}

#[test]
fn too_many_signatories_fails() {
	new_test_ext().execute_with(|| {
		let call = call_transfer(6, 15).encode();
		assert_noop!(
			Multisig::as_multi(
				Origin::signed(1),
				2,
				vec![2, 3, 4],
				None,
				OpaqueCall::from_encoded(call),
				false,
				0
			),
			Error::<Test>::TooManySignatories,
		);
	});
}

#[test]
fn duplicate_approvals_are_ignored() {
	new_test_ext().execute_with(|| {
		let call = call_transfer(6, 15).encode();
		let hash = blake2_256(&call);
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(1),
			2,
			vec![2, 3],
			None,
			hash.clone(),
			0
		));
		assert_noop!(
			Multisig::approve_as_multi(
				Origin::signed(1),
				2,
				vec![2, 3],
				Some(now()),
				hash.clone(),
				0
			),
			Error::<Test>::AlreadyApproved,
		);
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(2),
			2,
			vec![1, 3],
			Some(now()),
			hash.clone(),
			0
		));
		assert_noop!(
			Multisig::approve_as_multi(
				Origin::signed(3),
				2,
				vec![1, 2],
				Some(now()),
				hash.clone(),
				0
			),
			Error::<Test>::AlreadyApproved,
		);
	});
}

#[test]
fn multisig_1_of_3_works() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 1);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15).encode();
		let hash = blake2_256(&call);
		assert_noop!(
			Multisig::approve_as_multi(Origin::signed(1), 1, vec![2, 3], None, hash.clone(), 0),
			Error::<Test>::MinimumThreshold,
		);
		assert_noop!(
			Multisig::as_multi(
				Origin::signed(1),
				1,
				vec![2, 3],
				None,
				OpaqueCall::from_encoded(call),
				false,
				0
			),
			Error::<Test>::MinimumThreshold,
		);
		let boxed_call = Box::new(call_transfer(6, 15));
		assert_ok!(Multisig::as_multi_threshold_1(Origin::signed(1), vec![2, 3], boxed_call));

		assert_eq!(Balances::free_balance(6), 15);
	});
}

#[test]
fn multisig_filters() {
	new_test_ext().execute_with(|| {
		let call = Box::new(Call::System(frame_system::Call::set_code { code: vec![] }));
		assert_noop!(
			Multisig::as_multi_threshold_1(Origin::signed(1), vec![2], call.clone()),
			DispatchError::from(frame_system::Error::<Test>::CallFiltered),
		);
	});
}

#[test]
fn weight_check_works() {
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 2);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15);
		let data = call.encode();
		assert_ok!(Multisig::as_multi(
			Origin::signed(1),
			2,
			vec![2, 3],
			None,
			OpaqueCall::from_encoded(data.clone()),
			false,
			0
		));
		assert_eq!(Balances::free_balance(6), 0);

		assert_noop!(
			Multisig::as_multi(
				Origin::signed(2),
				2,
				vec![1, 3],
				Some(now()),
				OpaqueCall::from_encoded(data),
				false,
				0
			),
			Error::<Test>::MaxWeightTooLow,
		);
	});
}

#[test]
fn multisig_handles_no_preimage_after_all_approve() {
	// This test checks the situation where everyone approves a multi-sig, but no-one provides the
	// call data. In the end, any of the multisig callers can approve again with the call data and
	// the call will go through.
	new_test_ext().execute_with(|| {
		let multi = Multisig::multi_account_id(&[1, 2, 3][..], 3);
		assert_ok!(Balances::transfer(Origin::signed(1), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(2), multi, 5));
		assert_ok!(Balances::transfer(Origin::signed(3), multi, 5));

		let call = call_transfer(6, 15);
		let call_weight = call.get_dispatch_info().weight;
		let data = call.encode();
		let hash = blake2_256(&data);
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(1),
			3,
			vec![2, 3],
			None,
			hash.clone(),
			0
		));
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(2),
			3,
			vec![1, 3],
			Some(now()),
			hash.clone(),
			0
		));
		assert_ok!(Multisig::approve_as_multi(
			Origin::signed(3),
			3,
			vec![1, 2],
			Some(now()),
			hash.clone(),
			0
		));
		assert_eq!(Balances::free_balance(6), 0);

		assert_ok!(Multisig::as_multi(
			Origin::signed(3),
			3,
			vec![1, 2],
			Some(now()),
			OpaqueCall::from_encoded(data),
			false,
			call_weight
		));
		assert_eq!(Balances::free_balance(6), 15);
	});
}*/

