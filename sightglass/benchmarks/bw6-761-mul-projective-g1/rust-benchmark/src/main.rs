#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod bw6_761;
use utils::generate_arguments;
use bw6_761::do_mul_projective_g1;

fn main() {
    bench::start();
    let result = do_mul_projective_g1();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
