#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_scalar_args;
use ark_std::io::Error;

fn do_mul_affine_te(base: &ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_381_bandersnatch::SWConfig>, scalar: &[u64]) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_affine(
			base,
			scalar,
		);
	Ok(())
}

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig>>();
    bench::start();
    let result = do_mul_affine_te(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
