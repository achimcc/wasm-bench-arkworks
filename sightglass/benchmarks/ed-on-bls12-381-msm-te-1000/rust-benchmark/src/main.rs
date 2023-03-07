#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod ed_on_bls12_381;
use utils::generate_arguments;
use ed_on_bls12_381::do_msm_te;

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ed_on_bls12_381::EdwardsProjective>(1000);
    bench::start();
    let result = do_msm_te(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
