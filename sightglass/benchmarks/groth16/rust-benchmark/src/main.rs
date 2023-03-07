#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod groth16;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use groth16::do_verify_groth16;

fn main() {
    bench::start();
    let result = do_verify_groth16();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
