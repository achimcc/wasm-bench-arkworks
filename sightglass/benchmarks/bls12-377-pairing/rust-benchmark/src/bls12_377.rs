use ark_ec::{AffineRepr, pairing::Pairing};
use ark_std::io::Error;

pub fn do_pairing() -> Result<(), Error> {
	let _out = ark_bls12_377::Bls12_377::multi_pairing(
		[ark_bls12_377::G1Affine::generator()],
		[ark_bls12_377::G2Affine::generator()],
	);
	Ok(())
}