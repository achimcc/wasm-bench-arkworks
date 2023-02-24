#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod bls12_377;

fn main() {
    bench::start();
    let _ = bls12_377::do_msm_g1(10);
    bench::end();
}
