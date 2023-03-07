#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod bls12_377;
use utils::generate_arguments;
use bls12_377::do_mul_projective_g1;

fn main() {
    bench::start();
    let result = do_mul_projective_g1();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
