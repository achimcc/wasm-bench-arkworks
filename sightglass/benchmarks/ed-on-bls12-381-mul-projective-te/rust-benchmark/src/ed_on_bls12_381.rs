use ark_ec::{Group, twisted_edwards::TECurveConfig};
use ark_std::io::Error;

pub fn do_mul_projective_te() -> Result<(), Error> {
	let _out = <ark_ed_on_bls12_381::EdwardsConfig as TECurveConfig>::mul_projective(
		&ark_ed_on_bls12_381::EdwardsProjective::generator(),
		&[2u64],
	);
	Ok(())
}
