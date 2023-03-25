#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod utils;
use utils::generate_msm_args;
use ark_std::{io::Error, vec::Vec};

fn do_msm_g2(
	bases: &[ark_ec::short_weierstrass::Affine<ark_bw6_761::g2::Config>],
	scalars: &[<ark_bw6_761::g2::Config as ark_ec::CurveConfig>::ScalarField],
) -> Result<(), Error> {
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::msm(bases, scalars);

	Ok(())
}

fn main() {
    let (bases, scalars) = generate_msm_args::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>>(10);
    let bases = bases.iter().map(|base| base.into_affine()).collect::<Vec<_>>();
    bench::start();
    let result = do_msm_g2(&bases[..], &scalars[..]);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
