#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
use utils::{generate_arguments, bw6_761::do_msm_g2};
mod bw6_761;

fn main() {
    let (bases, scalars) = generate_arguments::<ark_ec::short_weierstrass::Projective<ark_bw6_761::g2::Config>>(1000);
    bench::start();
    let result = do_msm_g2(bases, scalars);
    bench::end();
    assert_eq!(result.unwrap(), ());
}
