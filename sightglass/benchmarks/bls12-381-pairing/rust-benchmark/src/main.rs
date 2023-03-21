#![cfg_attr(not(feature = "std"), no_std)]

mod bls12_381;
mod utils;
use sightglass_api as bench;
use utils::generate_pairing_args;
use ark_ec::CurveGroup;
use bls12_381::do_pairing;

fn main() {
    let (a, b) = generate_pairing_args::<ark_bls12_381::G1Affine, ark_bls12_381::G2Affine>();
    bench::start();
    let result = do_pairing(a, b);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
