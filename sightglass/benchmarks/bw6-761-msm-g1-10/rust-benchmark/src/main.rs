#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod bw6_761;

fn main() {
    bench::start();
    let result = bw6_761::do_msm_g1(10);
    bench::end();
}
