#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_scalar_args;
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_std::io::Error;

fn do_mul_projective_g2(
	base: &ark_ec::short_weierstrass::Affine<ark_bw6_761::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(&base, scalar);
	Ok(())
}

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>>();
    bench::start();
    let result = do_mul_projective_g2(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
