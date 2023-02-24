use ark_ec::{AffineRepr, Group, twisted_edwards::TECurveConfig};
use ark_std::io::Error;

pub fn do_mul_projective() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(
		&ark_ed_on_bls12_377::EdwardsProjective::generator(),
		&[2u64],
	);
	Ok(())
}
