use crate::{ import::* };


/// Constant used for rounding to a certain amount of decimal places.
/// 10000 = 4 decimal places.
//
const DIGITS: f64 = 10_000.0;

/// Represents an account balance. This add extra restrictions to f64.
/// A balance cannot be negative, cannot be Infinite nor NaN.
//
#[ derive( Debug, Copy, Clone, Default, PartialEq, PartialOrd ) ]
//
pub struct Balance( f64 );


/// An error to represent a float is not a valid balance.
//
#[ derive( Debug, Copy, Clone, PartialEq, Eq, Hash ) ]
//
pub enum FloatErr
{
	/// Bank balances are not allowed to be infinite.
	//
	Infinite,

	/// Bank balances are not allowed to be NaN.
	//
	NaN,

	/// Bank balances are not allowed to be negative.
	//
	Negative,
}


impl Balance
{
	/// Round the floating point number to a certain number of decimal places.
	/// This avoids a slow buildup of rounding error over repeated arithmetic operations.
	//
	fn round( &mut self ) -> Self
	{
		let rounded = ( self.0 * DIGITS ).round() / DIGITS;
		self.0 = rounded;
		*self
	}


	/// Try to addition two balances. Can fail if it causes overflow.
	//
	pub fn try_add( &self, rhs: Balance ) -> Result<Balance, FloatErr>
	{
		let mut res = Self::try_from( self.0 + rhs.0 )?;
		Ok( res.round() )
	}


	/// Try to substract balances. Can fail if the balance becomes negative.
	//
	pub fn try_sub( &self, rhs: Balance ) -> Result<Balance, FloatErr>
	{
		let mut res = Self::try_from( self.0 - rhs.0 )?;
		Ok( res.round() )
	}


	/// Convert to f64
	//
	pub fn to_f64( &self ) -> f64
	{
		self.0
	}
}


impl TryFrom<f64> for Balance
{
	type Error = FloatErr;

	fn try_from( f: f64 ) -> Result<Balance, FloatErr>
	{
		if f.is_sign_negative() { return Err(FloatErr::Negative) }
		if f.is_infinite     () { return Err(FloatErr::Infinite) }
		if f.is_nan          () { return Err(FloatErr::NaN     ) }

		Ok( Balance(f) )
	}
}


impl TryFrom<f32> for Balance
{
	type Error = FloatErr;

	fn try_from( f: f32 ) -> Result<Balance, FloatErr>
	{
		Self::try_from( Into::<f64>::into(f) )
	}
}


impl From<Balance> for f64
{
	fn from( b: Balance ) -> f64
	{
		b.0
	}
}


impl fmt::Display for Balance
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		self.0.fmt( f )
	}
}


impl PartialEq<f64> for Balance
{
	fn eq( &self, other: &f64 ) -> bool
	{
		self.0.eq( other )
	}
}
