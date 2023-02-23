#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod bls12_377;

fn main() {
    bench::start();
    let result = bls12_377::do_msm_g2(10);
    bench::end();
}
