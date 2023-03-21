use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ec::{pairing::Pairing, CurveConfig};
use ark_ff::Fp;
use ark_std::{io::Error, vec::Vec};

pub fn do_pairing(a: ark_bls12_381::G1Affine, b: ark_bls12_381::G2Affine) -> Result<(), Error> {
	let _ = ark_bls12_381::Bls12_381::multi_pairing([a], [b]);
	Ok(())
}

pub fn do_msm_g1(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config>],
	scalars: &[<ark_bls12_381::g1::Config as CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as ark_ec::models::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_msm_g2(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_381::g2::Config>],
	scalars: &[<ark_bls12_381::g2::Config as CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

pub fn do_mul_affine_g1(
	base: &ark_ec::short_weierstrass::Affine<ark_bls12_381::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		base, scalar,
	);
	Ok(())
}

pub fn do_mul_projective_g1(
	base: &ark_ec::short_weierstrass::Projective<ark_bls12_381::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_bls12_381::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base, scalar,
		);
	Ok(())
}

pub fn do_mul_affine_g2(
	base: &ark_ec::short_weierstrass::Affine<ark_bls12_381::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		base, scalar,
	);
	Ok(())
}

pub fn do_mul_projective_g2(
	base: &ark_ec::short_weierstrass::Projective<ark_bls12_381::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out =
		<ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			base, scalar,
		);
	Ok(())
}
