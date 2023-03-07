use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ec::{pairing::Pairing, AffineRepr, CurveConfig, Group};
use ark_ff::Fp;
use ark_std::{io::Error, vec::Vec};
use frame_support::assert_ok;

pub fn do_pairing() -> Result<(), Error> {
	let _ = ark_bls12_381::Bls12_381::multi_pairing(
		[ark_bls12_381::G1Affine::generator()],
		[ark_bls12_381::G2Affine::generator()],
	);
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

pub fn do_mul_affine_g1() -> Result<(), Error> {
	let _out = <ark_bls12_381::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		&ark_bls12_381::G1Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g1() -> Result<(), Error> {
	let _out =
		<ark_bls12_381::g1::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			&ark_bls12_381::G1Projective::generator(),
			&[2u64],
		);
	Ok(())
}

pub fn do_mul_affine_g2() -> Result<(), Error> {
	let _out = <ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_affine(
		&ark_bls12_381::G2Affine::generator(),
		&[2u64],
	);
	Ok(())
}

pub fn do_mul_projective_g2() -> Result<(), Error> {
	let _out =
		<ark_bls12_381::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::mul_projective(
			&ark_bls12_381::G2Projective::generator(),
			&[2u64],
		);
	Ok(())
}