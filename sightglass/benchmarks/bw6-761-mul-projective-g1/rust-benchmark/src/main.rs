#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
mod bw6_761;
use utils::generate_scalar_args;
use ark_ec::CurveGroup;
use bw6_761::do_mul_projective_g1;

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>>();
    bench::start();
    let result = do_mul_projective_g1(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
