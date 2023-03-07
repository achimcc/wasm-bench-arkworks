#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use crate::{utils::generate_arguments, bw6_761::do_pairing};

fn main() {
    bench::start();
    let result = do_pairing();
    bench::end();
    assert_eq!(result.unwrap(), ());
}
