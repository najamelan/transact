//! utility functions.



pub(crate) enum FloatErr
{
	Infinite,
	NaN,
	Negative,
}


/// Indicates whether a float is non-negative and is not INFINITY, nor NAN.
//
pub(crate) fn validate_float( f: f64 ) -> Result<f64, FloatErr>
{
	if f.is_sign_negative() { return Err(FloatErr::Negative) }
	if f.is_infinite     () { return Err(FloatErr::Infinite) }
	if f.is_nan          () { return Err(FloatErr::NaN     ) }

	Ok(f)
}
