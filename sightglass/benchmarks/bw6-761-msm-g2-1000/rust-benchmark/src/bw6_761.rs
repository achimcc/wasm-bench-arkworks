use ark_ec::{AffineRepr, short_weierstrass::SWCurveConfig};
use ark_std::{io::Error, vec::Vec};

pub fn do_msm_g2(samples: u32) -> Result<(), Error> {
	let g = ark_bw6_761::G2Affine::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples).map(|_| ark_ff::Fp::from(2u64)).collect();
	let _out = <ark_bw6_761::g2::Config as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}