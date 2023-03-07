#![cfg_attr(not(feature = "std"), no_std)]

mod bls12_381;
mod utils;
use sightglass_api as bench;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use bls12_381::do_pairing;

fn main() {
    bench::start();
    let result = do_pairing();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
