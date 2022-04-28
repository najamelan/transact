use crate::{ import::*, TransErr };

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

#[ derive( Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize) ]
//
pub enum TransState
{
	New,
	Success,
	Failed,
	Disputed,
	ChargedBack,
}

#[ derive( Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize) ]
//
pub struct Transact
{
	pub(crate) ttype : TransType,
	pub(crate) state : TransState,
	pub(crate) client: u16,
	pub(crate) id    : u32,
}


impl Transact
{
	pub(crate) fn new( ttype : TransType, client: u16, id: u32 ) -> Self
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
			( "deposit"   , Some(a) ) if a >= 0.0 => Ok( Transact{ state: TransState::New, ttype: TransType::Deposit (a), client: r.client, id: r.tx } ),
			( "withdrawal", Some(a) ) if a >= 0.0 => Ok( Transact{ state: TransState::New, ttype: TransType::WithDraw(a), client: r.client, id: r.tx } ),

			( "dispute"   , None ) => Ok( Transact{ state: TransState::New, ttype: TransType::Dispute   , client: r.client, id: r.tx } ),
			( "resolve"   , None ) => Ok( Transact{ state: TransState::New, ttype: TransType::Resolve   , client: r.client, id: r.tx } ),
			( "chargeback", None ) => Ok( Transact{ state: TransState::New, ttype: TransType::ChargeBack, client: r.client, id: r.tx } ),

			// TODO: add info about what's wrong.
			//
			_ => Err( TransErr::DeserializeTransact{ source: None } )
		}
	}
}
