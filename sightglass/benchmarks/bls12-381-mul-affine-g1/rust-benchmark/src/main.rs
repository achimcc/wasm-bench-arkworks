#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use utils::{generate_arguments, bls12_381::do_mul_affine_g1};

fn main() {
    bench::start();
    let result = bls12_381::do_mul_affine_g1();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
