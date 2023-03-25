#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_msm_args;
use ark_std::{io::Error, vec::Vec};
use ark_ec::CurveGroup;

fn do_msm_te(
	bases: &[ark_ec::twisted_edwards::Affine<ark_ed_on_bls12_381::SWConfig>],
	scalars: &[<ark_ed_on_bls12_381::EdwardsConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::JubjubConfig as ark_ec::twisted_edwards::TECurveConfig>::msm(
		bases, scalars,
	);
	Ok(())
}

fn main() {
    let (bases, scalars) = generate_msm_args::<ark_ed_on_bls12_381::EdwardsProjective>(10);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = do_msm_te(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
