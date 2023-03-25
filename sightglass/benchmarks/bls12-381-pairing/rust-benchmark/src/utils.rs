#![cfg_attr(not(feature = "std"), no_std)]

use ark_std::{test_rng, UniformRand};

fn generate_base<Group: UniformRand>() -> Group {
	let rng = &mut test_rng();
	let base = Group::rand(rng);
	base
}

pub fn generate_pairing_args<
	GroupA: UniformRand,
	GroupB: UniformRand,
>() -> (GroupA, GroupB) {
	(generate_base::<GroupA>(), generate_base::<GroupB>())
}

