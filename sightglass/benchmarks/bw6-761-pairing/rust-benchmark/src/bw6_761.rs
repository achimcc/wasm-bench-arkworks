use ark_ec::{AffineRepr, pairing::Pairing};
use ark_std::io::Error;

pub fn do_pairing() -> Result<(), Error> {
	let _out = ark_bw6_761::BW6_761::multi_pairing(
		[ark_bw6_761::G1Affine::generator()],
		[ark_bw6_761::G2Affine::generator()],
	);
	Ok(())
}
