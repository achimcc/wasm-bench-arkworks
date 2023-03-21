#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod bw6_761;
use utils::generate_msm_args;
use ark_ec::CurveGroup;
use bw6_761::do_msm_g1;

fn main() {
    let (bases, scalars) = generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>>(10);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = do_msm_g1(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
