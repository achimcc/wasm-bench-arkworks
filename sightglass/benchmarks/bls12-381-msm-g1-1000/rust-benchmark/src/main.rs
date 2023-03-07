#![cfg_attr(not(feature = "std"), no_std)]

mod bls12_381;
mod utils;
use sightglass_api as bench;
use utils::generate_arguments;
use bls12_381::do_msm_g1;

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>>(1000);
    bench::start();
    let result = do_msm_g1(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
