#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod bls12_377;
use utils::generate_scalar_args;
use ark_ec::CurveGroup;
use bls12_377::do_mul_affine_g2;

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ec::short_weierstrass::Affine<ark_bls12_377::g2::Config>>();
    bench::start();
    let result = do_mul_affine_g2(base, scalar);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
