#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use utils::{generate_arguments, ed_on_bls12_377::do_msm};

fn main() {
    let (bases, scalars) = generate_arguments::<sark_ed_on_bls12_377::EdwardsProjective>(10);
    bench::start();
    let result = do_msm(bases, scalars);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
