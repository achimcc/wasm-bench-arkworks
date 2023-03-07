#![cfg_attr(not(feature = "std"), no_std)]

mod bls12_381;
mod utils;
use sightglass_api as bench;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use bls12_381::do_msm_g2;

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>>(10);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = do_msm_g2(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
