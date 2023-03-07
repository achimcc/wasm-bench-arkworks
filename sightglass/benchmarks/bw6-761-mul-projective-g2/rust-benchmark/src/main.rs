#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use crate::{utils::generate_arguments, bw6_761::do_mul_projective_g2};

fn main() {
    bench::start();
    let result = do_mul_projective_g2();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
