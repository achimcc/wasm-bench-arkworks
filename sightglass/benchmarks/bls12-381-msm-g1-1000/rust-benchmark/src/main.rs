#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod bls12_381;

fn main() {
    bench::start();
    let result = bls12_381::do_msm_g1(1000);
    bench::end();
}
