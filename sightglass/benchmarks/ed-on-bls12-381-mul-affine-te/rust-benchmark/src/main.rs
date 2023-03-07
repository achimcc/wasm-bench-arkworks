#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod ed_on_bls12_381;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use ed_on_bls12_381::do_mul_affine_te;

fn main() {
    bench::start();
    let result = do_mul_affine_te();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
