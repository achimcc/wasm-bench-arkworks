#![cfg_attr(not(feature = "std"), no_std)]

use ark_std::{test_rng, UniformRand};

fn generate_scalar() ->u64 {
	let rng = &mut test_rng();
	let scalar = u64::rand(rng);
	scalar
}

fn generate_base<Group: UniformRand>() -> Group {
	let rng = &mut test_rng();
	let base = Group::rand(rng);
	base
}

pub fn generate_scalar_args<Group: UniformRand>() -> (Group, u64) {
	let serialized_scalar = generate_scalar();
	let serialized_base = generate_base::<Group>();
	(serialized_base, serialized_scalar)
}

