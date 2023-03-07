#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod ed_on_bls12_377;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use ed_on_bls12_377::do_mul_affine;

fn main() {
    bench::start();
    let result = do_mul_affine();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
