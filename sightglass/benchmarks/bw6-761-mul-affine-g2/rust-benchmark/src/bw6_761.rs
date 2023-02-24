use ark_ec::{AffineRepr, short_weierstrass::SWCurveConfig};
use ark_std::io::Error;

pub fn do_mul_affine_g2() -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_affine(
		&ark_bw6_761::G2Affine::generator(),
		&[2u64],
	);
	Ok(())
}