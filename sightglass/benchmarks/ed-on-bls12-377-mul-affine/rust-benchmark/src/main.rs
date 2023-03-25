#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_scalar_args;
use ark_std::io::Error;
use ark_ec::models::twisted_edwards::TECurveConfig;

fn do_mul_affine(
	base: &ark_ed_on_bls12_377::EdwardsAffine,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_affine(
			base,
			scalar,
		);
	Ok(())
}

fn main() {
    let (base, scalar)=generate_scalar_args::<ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_377::EdwardsConfig>>();
    bench::start();
    let result = do_mul_affine(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
