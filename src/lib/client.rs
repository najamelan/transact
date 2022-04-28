
/// Constant used for rounding to a certain amount of decimal places.
/// 10000 = 4 decimal places.
//
const DIGITS: f64 = 10_000.0;


#[ derive( Copy, Clone, PartialEq, PartialOrd, Debug ) ]
//
pub struct Client
{
	available: f64,
	held     : f64,
	id       : u16,
	locked   : bool,
}


impl Client
{
	pub fn new( id: u16 ) -> Self
	{
		Self
		{
			available: 0.0  ,
			held     : 0.0  ,
			locked   : false,
			id              ,
		}
	}


	/// Round the floating point number to a certain number of decimal places.
	/// This avoids a slow buildup of rounding error over repeated arithmetic operations.
	//
	fn round( n: f64 ) -> f64
	{
		(n * DIGITS).round() / DIGITS
	}


	/// Update the available funds.
	//
	pub(crate) fn set_available( &mut self, new: f64 ) -> &mut Self
	{
		self.available = Self::round( new );
		self
	}


	/// Update the available funds.
	//
	pub(crate) fn set_held( &mut self, new: f64 ) -> &mut Self
	{
		self.held = Self::round( new );
		self
	}


	/// The unique identifier for this account.
	//
	pub fn id( &self ) -> u16
	{
		self.id
	}


	/// The available funds for the client. These are the funds they dispose of
	/// for withdrawal.
	//
	pub fn available( &self ) -> f64
	{
		self.available
	}


	/// Disputed funds are funds for a deposit the client wishes to undo.
	//
	pub fn held( &self ) -> f64
	{
		self.held
	}


	/// The total of available and disputed funds.
	//
	pub fn total( &self ) -> f64
	{
		Self::round( self.available + self.held )
	}


	/// After a charge-back the account will be locked and no further transactions
	/// will be allowed.
	//
	pub fn is_locked( &self ) -> bool
	{
		self.locked
	}


	/// Lock this account. No further transactions will be allowed.
	//
	pub fn lock( &mut self ) -> &mut Self
	{
		self.locked = true;
		self
	}
}
