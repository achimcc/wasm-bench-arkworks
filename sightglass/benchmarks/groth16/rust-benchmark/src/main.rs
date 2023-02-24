#![cfg_attr(not(feature = "std"), no_std)]

use sightglass_api as bench;
mod groth16;

fn main() {
    bench::start();
    let _ = groth16::do_verify_groth16();
    bench::end();
}
