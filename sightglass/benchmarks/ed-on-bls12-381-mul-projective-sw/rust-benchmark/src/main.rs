#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use utils::{generate_arguments, ed_on_bls12_381::do_mul_projective_sw};

fn main() {
    bench::start();
    do_mul_projective_sw();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
