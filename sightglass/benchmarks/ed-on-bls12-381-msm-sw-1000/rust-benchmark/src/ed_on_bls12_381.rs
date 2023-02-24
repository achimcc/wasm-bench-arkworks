use ark_ec::{AffineRepr, twisted_edwards::TECurveConfig};
use ark_std::{io::Error, vec::Vec};

pub fn do_msm_sw(samples: u32) -> Result<(), Error> {
	let g = ark_ed_on_bls12_381::SWAffine::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples).map(|_| ark_ff::Fp::from(2u32)).collect();
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::msm(&v[..], &scalars[..]);
	Ok(())
}
