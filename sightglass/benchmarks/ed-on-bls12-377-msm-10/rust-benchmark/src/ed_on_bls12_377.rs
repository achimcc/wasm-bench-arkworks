use ark_ec::{AffineRepr, models::twisted_edwards::TECurveConfig};
use ark_std::{io::Error, vec::Vec};

pub fn do_msm(samples: u32) -> Result<(), Error> {
	let g = ark_ed_on_bls12_377::EdwardsAffine::generator();
	let v: Vec<_> = (0..samples).map(|_| g).collect();
	let scalars: Vec<_> = (0..samples).map(|_| ark_ff::Fp::from(2u64)).collect();
	let _out =
		<ark_ed_on_bls12_377::EdwardsConfig as TECurveConfig>::msm(
			&v[..],
			&scalars[..],
		);
	Ok(())
}
