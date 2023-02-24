use ark_ec::{Group, short_weierstrass::SWCurveConfig};
use ark_std::io::Error;

pub fn do_mul_projective_g2() -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_projective(
		&ark_bw6_761::G2Projective::generator(),
		&[2u64],
	);
	Ok(())
}
