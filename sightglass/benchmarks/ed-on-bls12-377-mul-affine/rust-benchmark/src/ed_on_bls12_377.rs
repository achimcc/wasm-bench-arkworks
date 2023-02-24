use ark_ec::{AffineRepr, Group, models::twisted_edwards::TECurveConfig};
use ark_std::io::Error;

pub fn do_mul_affine() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::msm(
			&[ark_ed_on_bls12_377::EdwardsAffine::generator()],
			&[2u64.into()],
		);
	Ok(())
}
