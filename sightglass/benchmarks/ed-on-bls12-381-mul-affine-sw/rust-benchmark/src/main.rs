#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use crate::{utils::generate_arguments,ed_on_bls12_381::do_mul_affine_sw};

fn main() {
    bench::start();
    let result = do_mul_affine_sw();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
