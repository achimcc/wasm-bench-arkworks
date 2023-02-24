use ark_ec::{AffineRepr, Group, short_weierstrass::SWCurveConfig};
use ark_std::io::Error;

pub fn do_mul_projective_sw() -> Result<(), Error> {
	let _out =
		<ark_ed_on_bls12_381::EdwardsConfig as SWCurveConfig>::mul_projective(
			&ark_ed_on_bls12_381::SWProjective::generator(),
			&[2u64],
		);
	Ok(())
}
