#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod ed_on_bls12_381;
use utils::generate_scalar_args;
use ark_ec::CurveGroup;
use ed_on_bls12_381::do_mul_affine_te;

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_381::EdwardsConfig>>();
    bench::start();
    let result = do_mul_affine_te(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}