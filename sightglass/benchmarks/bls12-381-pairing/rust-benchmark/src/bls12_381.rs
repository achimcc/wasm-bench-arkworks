use ark_ec::{AffineRepr, pairing::Pairing};
use ark_std::io::Error;

pub fn do_pairing() -> Result<(), Error> {
	let _result = ark_bls12_381::Bls12_381::multi_pairing(
		[ark_bls12_381::G1Affine::generator()],
		[ark_bls12_381::G2Affine::generator()],
	);
	Ok(())
}
