#![cfg_attr(not(feature = "std"), no_std)]

mod utils;
use sightglass_api as bench;
use utils::generate_pairing_args;
use ark_std::io::Error;

fn do_pairing(a: ark_bls12_381::G1Affine, b: ark_bls12_381::G2Affine) -> Result<(), Error> {
	let _ = ark_bls12_381::Bls12_381::multi_pairing([a], [b]);
	Ok(())
}

fn main() {
    let (a, b) = generate_pairing_args::<ark_bls12_381::G1Affine, ark_bls12_381::G2Affine>();
    bench::start();
    let result = do_pairing(a, b);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
