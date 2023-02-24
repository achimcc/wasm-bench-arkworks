use ark_ec::{short_weierstrass::SWCurveConfig, Group};
use ark_std::io::Error;

pub fn do_mul_projective_g1() -> Result<(), Error> {
	let _out = <ark_bls12_377::g1::Config as SWCurveConfig>::mul_projective(
		&ark_bls12_377::G1Projective::generator(),
		&[2u64],
	);
	Ok(())
}