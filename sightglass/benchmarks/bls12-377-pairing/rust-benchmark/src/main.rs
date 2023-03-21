#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod bls12_377;
use utils::generate_pairing_args;
use ark_ec::CurveGroup;
use bls12_377::do_pairing;

fn main() {
    let (a, b) = generate_pairing_args::<ark_bls12_377::G1Affine, ark_bls12_377::G2Affine>();
    bench::start();
    let result = do_pairing(a, b);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
