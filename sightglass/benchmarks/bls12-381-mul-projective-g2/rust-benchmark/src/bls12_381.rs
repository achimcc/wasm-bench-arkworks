use ark_ec::{Group, short_weierstrass::SWCurveConfig};
use ark_std::io::Error;

pub fn do_mul_projective_g2() -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as SWCurveConfig>::mul_projective(
		&ark_bls12_381::G2Projective::generator(),
		&[2u64],
	);
	Ok(())
}
