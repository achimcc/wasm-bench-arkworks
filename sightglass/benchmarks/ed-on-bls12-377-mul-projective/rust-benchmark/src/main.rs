#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_scalar_args;
use ark_std::io::Error;
use ark_ec::models::twisted_edwards::TECurveConfig;

fn do_mul_projective(
	base: &ark_ed_on_bls12_377::EdwardsProjective,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(&base, scalar);
	Ok(())
}

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_377::EdwardsProjective>();
    bench::start();
    let result = do_mul_projective(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
