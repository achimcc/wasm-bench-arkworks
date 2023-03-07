#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use crate::{utils::generate_arguments, ed_on_bls12_381::do_msm_te};

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ed_on_bls12_381::EdwardsProjective>(10);
    bench::start();
    let result = ed_on_bls12_381::do_msm_te(bases, scalars);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
