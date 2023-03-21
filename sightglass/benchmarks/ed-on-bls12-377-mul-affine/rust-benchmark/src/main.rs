#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod ed_on_bls12_377;
use utils::generate_scalar_args;
use ark_ec::CurveGroup;
use ed_on_bls12_377::do_mul_affine;

fn main() {
    let (base, scalar)=generate_scalar_args::<ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_377::EdwardsConfig>>();
    bench::start();
    let result = do_mul_affine(&base, scalar);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
