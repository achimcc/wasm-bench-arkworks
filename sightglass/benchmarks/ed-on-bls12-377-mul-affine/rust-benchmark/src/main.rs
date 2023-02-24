#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod ed_on_bls12_377;

fn main() {
    bench::start();
    let _ = ed_on_bls12_377::do_mul_affine();
    bench::end();
}
