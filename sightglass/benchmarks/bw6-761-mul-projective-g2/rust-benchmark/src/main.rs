#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod bw6_761;

fn main() {
    bench::start();
    let _ = bw6_761::do_mul_projective_g2();
    bench::end();
}
