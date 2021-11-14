use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use alloc::string::String;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(CryptoModule::verify_sr25519(Origin::signed(1), [0; 32], String::from("hello"), [0; 64]));
	});
}
