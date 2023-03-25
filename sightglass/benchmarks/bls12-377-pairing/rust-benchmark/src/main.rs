#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_pairing_args;
use ark_std::io::Error;
use ark_ec::pairing::Pairing;

fn do_pairing(a: ark_bls12_377::G1Affine, b: ark_bls12_377::G2Affine) -> Result<(), Error> {
	let _out = ark_bls12_377::Bls12_377::multi_pairing([a], [b]);
	Ok(())
}

fn main() {
    let (a, b) = generate_pairing_args::<ark_bls12_377::G1Affine, ark_bls12_377::G2Affine>();
    bench::start();
    let result = do_pairing(a, b);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
