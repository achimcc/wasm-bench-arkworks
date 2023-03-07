#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use crate::{utils::generate_arguments,ed_on_bls12_377::do_mul_projective};

fn main() {
    bench::start();
    let result = do_mul_projective();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
