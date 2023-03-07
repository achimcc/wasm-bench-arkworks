#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod ed_on_bls12_381;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use ed_on_bls12_381::do_msm_sw;

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381::EdwardsConfig>>(1000);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = ed_on_bls12_381::do_msm_sw(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
