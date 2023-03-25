#![cfg_attr(not(feature = "std"), no_std)]

mod utils;
use sightglass_api as bench;
use utils::generate_msm_args;
use ark_ec::CurveGroup;
use ark_std::{io::Error, vec::Vec};

fn do_msm_g2(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bls12_377::g2::Config>],
	scalars: &[<ark_bls12_377::g2::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bls12_377::g2::Config as ark_ec::short_weierstrass::SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

fn main() {
    let (bases, scalars) = generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bls12_377::g2::Config>>(10);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = do_msm_g2(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
