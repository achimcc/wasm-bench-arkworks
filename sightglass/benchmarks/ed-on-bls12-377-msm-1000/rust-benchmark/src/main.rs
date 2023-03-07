#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod ed_on_bls12_377;
use utils::generate_arguments;
use ark_ec::CurveGroup;
use ed_on_bls12_377::do_msm;

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ed_on_bls12_377::EdwardsProjective>(1000);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = do_msm(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
