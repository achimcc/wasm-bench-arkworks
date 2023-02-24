use ark_ec::{AffineRepr, short_weierstrass::SWCurveConfig};
use ark_std::io::Error;

pub fn do_mul_affine_sw() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_affine(
		&ark_ed_on_bls12_381::SWAffine::generator(),
		&[2u64],
	);
	Ok(())
}
