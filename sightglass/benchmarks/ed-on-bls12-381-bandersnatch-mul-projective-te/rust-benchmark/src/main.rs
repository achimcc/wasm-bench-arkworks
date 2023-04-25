#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_scalar_args;
use ark_std::io::Error;

fn do_mul_projective_te(
	base: &ark_ec::twisted_edwards::Projective<ark_ed_on_bls12_381_bandersnatch::JubjubConfig>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		base,
		scalar,
	);
	Ok(())
}

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_381_bandersnatch::EdwardsProjective>();
    bench::start();
    let result = do_mul_projective_te(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
