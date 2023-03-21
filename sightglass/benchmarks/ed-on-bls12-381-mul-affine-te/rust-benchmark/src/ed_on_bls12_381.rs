use ark_std::{io::Error, vec::Vec};
use ark_ec::short_weierstrass::SWCurveConfig;

pub fn do_msm_sw(
	bases: &[ark_ec::short_weierstrass::Affine<ark_ed_on_bls12_381::SWConfig>],
	scalars: &[<ark_ed_on_bls12_381::SWConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_msm_te(
	bases: &[EdwardsAffineOptimized],
	scalars: &[<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::JubjubConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_mul_affine_sw(base: &ark_ed_on_bls12_381::SWAffine, scalar: &[u64]) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_affine(&base, scalar);
	Ok(())
}

pub fn do_mul_affine_te(base: &EdwardsAffineOptimized, scalar: &[u64]) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_affine(
			base,
			scalar,
		);
	Ok(())
}

pub fn do_mul_projective_sw(
	base: &ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381::SWConfig>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base,
			scalar,
		);
	Ok(())
}

pub fn do_mul_projective_te(
	base: &ark_ec::twisted_edwards::Projective<ark_ed_on_bls12_381::JubjubConfig>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		base,
		scalar,
	);
	Ok(())
}
