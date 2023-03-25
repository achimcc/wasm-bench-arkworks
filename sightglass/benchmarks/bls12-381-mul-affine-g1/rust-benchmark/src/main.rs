#![cfg_attr(not(feature = "std"), no_std)]

mod utils;
use sightglass_api as bench;
use utils::generate_scalar_args;
use ark_std::{io::Error, vec::Vec};

fn do_mul_affine_g1(
	base: &ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		base, scalar,
	);
	Ok(())
}

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config>>();
    bench::start();
    let result = bls12_381::do_mul_affine_g1(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
