#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod bw6_761;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use bw6_761::do_pairing;

fn main() {
    bench::start();
    let result = do_pairing();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
