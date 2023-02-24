use ark_ec::{AffineRepr, Group, short_weierstrass::SWCurveConfig, pairing::Pairing};
use ark_std::{io::Error, vec::Vec};

pub fn do_pairing() -> Result<(), Error> {
	let _out = ark_bls12_377::Bls12_377::multi_pairing(
		[ark_bls12_377::G1Affine::generator()],
		[ark_bls12_377::G2Affine::generator()],
	);
	Ok(())
}