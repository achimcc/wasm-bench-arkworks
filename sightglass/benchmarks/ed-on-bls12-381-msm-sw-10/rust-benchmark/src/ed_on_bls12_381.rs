use ark_std::{io::Error, vec::Vec};
use ark_ec::{pairing::Pairing, AffineRepr, CurveConfig, Group};

pub fn do_msm_sw(
	bases: &[ark_e::short_weierstrass::Affine<ark_ed_on_bls12_381::SWConfig>],
	scalars: &[<ark_ed_on_bls12_381::SWConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::msm(&bases[..], &scalars[..]);
	Ok(())
}

pub fn do_msm_te(
	bases: &[ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_381::JubjubConfig>],
	scalars: &[<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::JubjubConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_mul_affine_sw() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		&ark_ed_on_bls12_381::SWAffine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_affine_te() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as ark_e::short_weierstrass::SWCurveConfig>::mul_affine(
			&ark_ed_on_bls12_381::SWAffine::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_projective_sw() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as ark_e::short_weierstrass::SWCurveConfig>::mul_projective(
			&ark_ed_on_bls12_381::SWProjective::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_projective_te() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as ark_ec::twisted_edwards::TECurveConfig>::mul_projective(
		&ark_ed_on_bls12_381::EdwardsProjective::generator(),
		&[2u64],
	);
	Ok(())
}
