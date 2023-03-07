#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use utils::{generate_arguments, bls12_381::groth16};

fn main() {
    bench::start();
    let result = do_verify_groth16();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
