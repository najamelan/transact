use crate::{ * };





/// Represents a client account.
/// There are 2 types of balance: available and held.
/// Held corresponds to funds from disputed transactions.
//
#[ derive( Copy, Clone, PartialEq, Debug ) ]
//
pub struct Client
{
	available: Balance ,
	held     : Balance ,
	id       : u16     ,
	locked   : bool    ,
}


impl Client
{
	/// Create a new client with the given ID.
	//
	pub fn new( id: u16 ) -> Self
	{
		Self
		{
			available: Balance::try_from( 0.0 ).unwrap() ,
			held     : Balance::try_from( 0.0 ).unwrap() ,
			locked   : false                             ,
			id                                           ,
		}
	}


	/// The unique identifier for this account.
	//
	pub fn id( &self ) -> u16
	{
		self.id
	}


	/// Update the available balance.
	///
	/// This can fail if the total of held and available balances cannot be represented
	/// with a float.
	//
	pub fn set_available( &mut self, new: Balance ) -> Result<&mut Self, FloatErr>
	{
		self.held.try_add( new )?;

		self.available = new;
		Ok(self)
	}


	/// Update the held balance
	///
	/// This can fail if the total of held and available balances cannot be represented
	/// with a float.
	//
	pub fn set_held( &mut self, new: Balance ) -> Result<&mut Self, FloatErr>
	{
		self.held.try_add( new )?;

		self.held = new;
		Ok(self)
	}


	/// The available funds for the client. These are the funds they dispose of
	/// for withdrawal.
	//
	pub fn available( &self ) -> Balance
	{
		self.available
	}


	/// Disputed funds are funds for a deposit the client wishes to undo.
	//
	pub fn held( &self ) -> Balance
	{
		self.held
	}


	/// The total of available and disputed funds.
	/// This is infallible because the setters for available and held guarantee the sum
	/// can be represented in an f64.
	//
	pub fn total( &self ) -> Balance
	{
		self.available.try_add( self.held ).unwrap()
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
