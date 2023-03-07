#![cfg_attr(not(feature = "std"), no_std)]

use ark_serialize::{CanonicalSerialize, Compress};
use ark_std::{io::Cursor, test_rng, vec, vec::Vec, UniformRand};

pub mod bls12_377;
pub mod bls12_381;
pub mod bw6_761;
pub mod ed_on_bls12_377;
pub mod ed_on_bls12_381;

pub fn generate_arguments<Group: ark_ec::VariableBaseMSM>(
	size: u32,
) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
	let rng = &mut test_rng();
	let scalars = (0..size).map(|_| Group::ScalarField::rand(rng)).collect::<Vec<_>>();
	let bases = (0..size).map(|_| Group::rand(rng)).collect::<Vec<_>>();
	let bases = bases
		.iter()
		.map(|base| {
			let mut serialized_base = vec![0u8; base.serialized_size(Compress::No)];
			let mut cursor = Cursor::new(&mut serialized_base[..]);
			base.serialize_uncompressed(&mut cursor).unwrap();
			serialized_base
		})
		.collect::<Vec<_>>();
	let scalars = scalars
		.iter()
		.map(|scalar| {
			let mut serialized_scalar = vec![0u8; scalar.serialized_size(Compress::No)];
			let mut cursor = Cursor::new(&mut serialized_scalar[..]);
			scalar.serialize_uncompressed(&mut cursor).unwrap();
			serialized_scalar
		})
		.collect::<Vec<_>>();
	(bases, scalars)
}

