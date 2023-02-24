use ark_ec::{AffineRepr, Group, short_weierstrass::SWCurveConfig, pairing::Pairing};
use ark_std::{io::Error, vec::Vec};

pub fn do_pairing() -> Result<(), Error> {
	let _ = ark_bls12_381::Bls12_381::multi_pairing(
		[ark_bls12_381::G1Affine::generator()],
		[ark_bls12_381::G2Affine::generator()],
	);
	Ok(())
}
