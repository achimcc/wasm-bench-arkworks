#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use utils::{generate_arguments, ed_on_bls12_381::do_mul_projective_te};

fn main() {
    bench::start();
    let result = do_mul_projective_te();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
