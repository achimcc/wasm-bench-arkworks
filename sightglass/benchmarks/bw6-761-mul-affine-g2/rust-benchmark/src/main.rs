#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use utils::{generate_arguments,bw6_761::do_mul_affine_g2};
mod bw6_761;

fn main() {
    bench::start();
    let result = do_mul_affine_g2();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
