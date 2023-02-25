#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod ed_on_bls12_381;

fn main() {
    bench::start();
    let result = ed_on_bls12_381::do_mul_projective_sw();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
