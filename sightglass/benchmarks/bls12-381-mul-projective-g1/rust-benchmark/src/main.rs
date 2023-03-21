#![cfg_attr(not(feature = "std"), no_std)]

mod bls12_381;
mod utils;
use sightglass_api as bench;
use utils::generate_scalar_args;
use ark_ec::CurveGroup;
use bls12_381::do_mul_projective_g1;

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>>();
    bench::start();
    let result = do_mul_projective_g1(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
