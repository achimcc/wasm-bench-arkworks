#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use crate::{utils::generate_arguments, bls12_377::do_mul_affine_g1};

fn main() {
    bench::start();
    let result = do_mul_affine_g1();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
