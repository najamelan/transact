use crate::{ import::*, TransErr };

/// The type of transaction.
//
#[ allow(missing_docs) ]
#[ derive( Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize) ]
//
pub enum TransType
{
	Deposit(f64),
	WithDraw(f64),
	Dispute,
	Resolve,
	ChargeBack,
}


/// The transaction state.
//
#[ derive( Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize) ]
//
pub enum TransState
{
	/// The transaction has not yet been processed.
	//
	New,

	/// The deposit or withdrawal has successfully been applied to the account balance.
	//
	Success,

	// currently we are not keeping track of failed transactions. They are not stored in the
	// database and are just output in error messages.
	//
	// Failed,

	/// Applies to deposit only, has been disputed. The funds are in "held".
	//
	Disputed,

	/// Applies to deposit only, a charge back has been applied to this deposit.
	//
	ChargedBack,
}


/// Internal representation of a transaction.
//
#[ allow(missing_docs) ]
#[ derive( Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize) ]
//
pub struct Transact
{
	pub ttype : TransType,
	pub state : TransState,
	pub client: u16,
	pub id    : u32,
}


impl Transact
{
	/// Create a new transaction. public for testing purposes only.
	//
	pub fn new( ttype : TransType, client: u16, id: u32 ) -> Self
	{
		Self
		{
			ttype,
			client,
			id,
			state: TransState::New,
		}
	}
}


#[ derive( Copy, Clone, Debug, Serialize, Deserialize) ]
//
pub(crate) struct CsvRecord<'a>
{
	r#type: &'a str,
	client: u16,
	tx    : u32,
	amount: Option<f64>,
}


impl<'a> TryFrom< CsvRecord<'a> > for Transact
{
	type Error = TransErr;

	fn try_from( r: CsvRecord<'a> ) -> Result<Transact, Self::Error>
	{
		match (r.r#type, r.amount)
		{
			// negative amounts are not valid.
			//
			( "deposit"   , Some(a) ) if a >= 0.0 => Ok( Transact::new( TransType::Deposit (a), r.client, r.tx ) ),
			( "withdrawal", Some(a) ) if a >= 0.0 => Ok( Transact::new( TransType::WithDraw(a), r.client, r.tx ) ),

			( "dispute"   , None ) => Ok( Transact::new( TransType::Dispute   , r.client, r.tx ) ),
			( "resolve"   , None ) => Ok( Transact::new( TransType::Resolve   , r.client, r.tx ) ),
			( "chargeback", None ) => Ok( Transact::new( TransType::ChargeBack, r.client, r.tx ) ),

			// TODO: add info about what's wrong.
			//
			_ => Err( TransErr::DeserializeTransact{ source: None } )
		}
	}
}
