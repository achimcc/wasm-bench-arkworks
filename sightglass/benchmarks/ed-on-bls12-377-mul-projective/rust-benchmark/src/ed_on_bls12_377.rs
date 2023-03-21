use ark_std::{io::Error, vec::Vec};
use ark_ec::TECurveConfig;

pub fn do_msm(
	bases: &[ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_377::EdwardsConfig>],
	scalars: &[<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::msm(
			bases, scalars,
		);
	Ok(())
}

pub fn do_mul_affine(
	base: &ark_ed_on_bls12_377::EdwardsAffine,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as ark_ec::models::twisted_edwards::TECurveConfig>::mul_affine(
			base,
			scalar,
		);
	Ok(())
}

pub fn do_mul_projective(
	base: &ark_ed_on_bls12_377::EdwardsProjective,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::mul_projective(base, scalar);
	Ok(())
}