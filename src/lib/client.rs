use crate::{ util::* };

/// Constant used for rounding to a certain amount of decimal places.
/// 10000 = 4 decimal places.
//
const DIGITS: f64 = 10_000.0;




/// Represents a client account.
/// There are 2 types of balance: available and held.
/// Held corresponds to funds from disputed transactions.
//
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
	/// Create a new client with the given ID.
	//
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
	pub(crate) fn update_balance( &mut self, available: f64, held: f64 ) -> Result<(), FloatErr>
	{
		let avail = match validate_float(available)
		{
			Ok(val) => val,

			x => return x.map( |a|
			{
				// No operations currently allow creating a negative account balance. This should never happen.
				//
				debug_assert!( !a.is_sign_negative(), "Trying to set a negative balance on client.available" );
			}),
		};


		let held = match validate_float(held)
		{
			Ok(val) => val,

			x => return x.map( |h|
			{
				// No operations currently allow creating a negative account balance. This should never happen.
				//
				debug_assert!( !h.is_sign_negative(), "Trying to set a negative balance on client.available" );
			}),
		};


		self.available = Self::round( avail );
		self.held      = Self::round( held  );

		Ok(())
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
