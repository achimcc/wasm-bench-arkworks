#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod bls12_377;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use bls12_377::do_msm_g2;

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bls12_377::g2::Config>>(1000);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = bls12_377::do_msm_g2(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
