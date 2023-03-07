#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use utils::{generate_arguments, ed_on_bls12_381::do_msm_sw};

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381::EdwardsConfig>>(10);
    bench::start();
    let result = do_msm_sw(bases, scalars);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
