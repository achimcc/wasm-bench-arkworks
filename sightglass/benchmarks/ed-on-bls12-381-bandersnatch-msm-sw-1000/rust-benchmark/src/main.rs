#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_msm_args;
use ark_std::{io::Error, vec::Vec};
use ark_ec::CurveGroup;

fn do_msm_sw(
	bases: &[ark_ec::short_weierstrass::Affine<ark_ed_on_bls12_381_bandersnatch::SWConfig>],
	scalars: &[<ark_ed_on_bls12_381_bandersnatch::SWConfig as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381_bandersnatch::EdwardsConfig as ark_ec::short_weierstrass::SWCurveConfig>::msm(bases, scalars);
	Ok(())
}

fn main() {
    let (bases, scalars) = generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_ed_on_bls12_381_bandersnatch::EdwardsConfig>>(1000);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = do_msm_sw(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
