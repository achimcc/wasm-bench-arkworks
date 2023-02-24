use ark_ec::{Group, short_weierstrass::SWCurveConfig};
use ark_std::io::Error;

pub fn do_mul_projective_g1() -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_projective(
		&ark_bw6_761::G1Projective::generator(),
		&[2u64],
	);
	Ok(())
}
