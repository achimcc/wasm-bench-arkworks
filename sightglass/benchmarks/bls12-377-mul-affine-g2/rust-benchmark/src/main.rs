#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod bls12_377;

fn main() {
    bench::start();
    let result = bls12_377::do_mul_affine_g2();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
