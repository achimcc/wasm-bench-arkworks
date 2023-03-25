#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_scalar_args;
use ark_std::io::Error;

fn do_mul_projective_sw(
	base: &ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381::SWConfig>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base,
			scalar,
		);
	Ok(())
}

fn main() {
    let (base, scalar) = generate_scalar_args::<ark_ed_on_bls12_381::SWProjective>();
    bench::start();
    let result = do_mul_projective_sw(&base, &[scalar]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
