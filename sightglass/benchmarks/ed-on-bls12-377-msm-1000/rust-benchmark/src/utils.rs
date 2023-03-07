#![cfg_attr(not(feature = "std"), no_std)]

use ark_std::{io::Cursor, test_rng, vec, vec::Vec, UniformRand};

pub fn generate_arguments<Group: ark_ec::VariableBaseMSM>(
	size: u32,
) -> (Vec<Group>, Vec<Group::ScalarField>) {
	let rng = &mut test_rng();
	let scalars = (0..size).map(|_| Group::ScalarField::rand(rng)).collect::<Vec<_>>();
	let bases = (0..size).map(|_| Group::rand(rng)).collect::<Vec<_>>();
	(bases, scalars)
}

