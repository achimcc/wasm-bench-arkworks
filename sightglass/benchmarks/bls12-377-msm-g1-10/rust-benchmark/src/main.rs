#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use utils::{generate_arguments, bls12_377::do_msm_g1};

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bls12_377::g1::Config>>(10);
    bench::start();
    let result = do_msm_g1(10);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
