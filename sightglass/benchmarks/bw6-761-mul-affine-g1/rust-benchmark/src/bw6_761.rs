use ark_std::{io::Error, vec::Vec};
pub use ark_ec::{pairing::Pairing, short_weierstrass::SWCurveConfig, AffineRepr, Group};

pub fn do_msm_g1(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bw6_761::g1::Config>],
	scalars: &[<ark_bw6_761::g1::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::msm(bases, scalars);

	Ok(())
}

pub fn do_msm_g2(
	bases: &[ark_ec::models::short_weierstrass::Affine<ark_bw6_761::g2::Config>],
	scalars: &[<ark_bw6_761::g2::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

pub fn do_mul_affine_g1(
	base: &ark_ec::short_weierstrass::Affine<ark_bw6_761::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_affine(&base, &scalar);
	Ok(())
}

pub fn do_mul_projective_g1(
	base: &ark_ec::short_weierstrass::Projective<ark_bw6_761::g1::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g1::Config as SWCurveConfig>::mul_projective(&base, &scalar);
	Ok(())
}

pub fn do_mul_affine_g2(
	base: &ark_ec::short_weierstrass::Affine<ark_bw6_761::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_affine(&base, &scalar);
	Ok(())
}
pub fn do_mul_projective_g2(
	base: &ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>,
	scalar: &[u64],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::mul_projective(&base, &scalar);
	Ok(())
}

pub fn do_pairing(a: ark_bw6_761::G1Affine, b: ark_bw6_761::G2Affine) -> Result<(), Error> {
	let _out = ark_bw6_761::BW6_761::multi_pairing([a], [b]);
	Ok(())
}
