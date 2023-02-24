use ark_ec::{AffineRepr, Group, short_weierstrass::SWCurveConfig};
use ark_std::io::Error;

pub fn do_mul_affine_g1() -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as SWCurveConfig>::mul_affine(
		&ark_bls12_381::G1Affine::generator(),
		&[2u64],
	);
	Ok(())
}
