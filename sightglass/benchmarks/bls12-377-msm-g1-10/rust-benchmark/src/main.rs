#![cfg_attr(not(feature = "std"), no_std)]

mod utils;
mod bls12_377;
use sightglass_api as bench;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use bls12_377::do_msm_g1;

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bls12_377::g1::Config>>(10);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = do_msm_g1(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
