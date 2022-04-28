

#[ derive( Copy, Clone, PartialEq, PartialOrd, Debug ) ]
//
pub struct Client
{
	pub(crate) available: f64,
	pub(crate) held     : f64,
	pub(crate) id       : u16,
	pub(crate) locked   : bool,
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


	pub fn total( &self ) -> f64
	{
		self.available + self.held
	}
}
