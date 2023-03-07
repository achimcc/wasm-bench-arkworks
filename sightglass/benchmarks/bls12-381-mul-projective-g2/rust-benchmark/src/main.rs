#![cfg_attr(not(feature = "std"), no_std)]

mod bls12_381;
mod utils;
use sightglass_api as bench;
use utils::generate_arguments;
use bls12_381::do_mul_projective_g2;

fn main() {
    bench::start();
    let result = do_mul_projective_g2();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
