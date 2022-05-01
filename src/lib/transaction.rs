use crate::{ import::*, * };

/// The type of transaction.
//
#[ allow(missing_docs) ]
#[ derive( Clone, PartialEq, PartialOrd, Debug) ]
//
pub enum TransType
{
	Deposit (BigDecimal) ,
	WithDraw(BigDecimal) ,
	Dispute              ,
	Resolve              ,
	ChargeBack           ,
}


impl fmt::Display for TransType
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> fmt::Result
	{
		match self
		{
			Self::Deposit (a) => write!( f, "Deposit({})" , a.normalized() ),
			Self::WithDraw(a) => write!( f, "WithDraw({})", a.normalized() ),
			Self::Dispute     => write!( f, "Dispute"                      ),
			Self::Resolve     => write!( f, "Resolve"                      ),
			Self::ChargeBack  => write!( f, "ChargeBack"                   ),
		}
	}
}


/// The transaction state.
//
#[ derive( Copy, Clone, PartialEq, PartialOrd, Debug) ]
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
#[ derive( Clone, PartialEq, Debug) ]
//
pub struct Transact
{
	pub ttype : TransType  ,
	pub state : TransState ,
	pub client: u16        ,
	pub id    : u32        ,
}


impl Transact
{
	/// Create a new transaction. public for testing purposes only.
	//
	pub fn new( ttype : TransType, client: u16, id: u32 ) -> Self
	{
		Self
		{
			ttype                  ,
			client                 ,
			id                     ,
			state: TransState::New ,
		}
	}
}


impl fmt::Display for Transact
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> fmt::Result
	{
		write!
		(
			f, "Transaction: type: {}, client: {}, tx: {}",
			self.ttype, self.client, self.id
		)
	}
}



/// The format actually in the CSV file.
/// Used for deserializing with Serde.
//
#[ derive( Debug, Deserialize) ]
//
pub struct CsvRecord<'a>
{
	r#type: Cow<'a, str>       ,
	client: u16                ,
	tx    : u32                ,
	amount: Option<BigDecimal> ,
}


impl CsvRecord<'_>
{
	fn to_owned( &self ) -> CsvRecord<'static>
	{
		CsvRecord
		{
			r#type: Cow::Owned( self.r#type.clone().into_owned() ),
			client: self.client,
			tx    : self.tx,
			amount: self.amount.clone(),
		}
	}
}


impl fmt::Display for CsvRecord<'_>
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> fmt::Result
	{
		write!
		(
			f, "CsvRecord: type: {}, client: {}, tx: {}, amount: {:?}",
			self.r#type, self.client, self.tx, self.amount.as_ref().map( BigDecimal::normalized )
		)
	}
}



impl<'a> TryFrom< CsvRecord<'a> > for Transact
{
	type Error = TransErr;

	fn try_from( r: CsvRecord<'a> ) -> Result<Transact, Self::Error>
	{
		let CsvRecord { r#type, client, tx, amount } = r;

		match (r#type.as_ref(), amount)
		{
			( x, Some(a) ) =>
			{
				if a.is_negative()
				{
					let record = CsvRecord{ r#type, client, tx, amount: Some(a) }.to_owned();
					return Err( TransErr::DeserializeTransact{ kind: DeserTransactKind::AmountNegative, record } );
				}

				let ttype = match x
				{
					"deposit"    => TransType::Deposit (a),
					"withdrawal" => TransType::WithDraw(a),

					_ =>
					{
						let record = CsvRecord{ r#type, client, tx, amount: Some(a) }.to_owned();
						return Err( TransErr::DeserializeTransact{ kind: DeserTransactKind::AmountNegative, record } );
					}
				};

				Ok( Transact::new( ttype, r.client, r.tx ) )
			}

			( "dispute"   , None ) => Ok( Transact::new( TransType::Dispute   , r.client, r.tx ) ),
			( "resolve"   , None ) => Ok( Transact::new( TransType::Resolve   , r.client, r.tx ) ),
			( "chargeback", None ) => Ok( Transact::new( TransType::ChargeBack, r.client, r.tx ) ),

			( _, None ) =>
			{
				let record = CsvRecord{ r#type, client, tx, amount: None }.to_owned();
				Err( TransErr::DeserializeTransact{ kind: DeserTransactKind::AmountNegative, record } )
			}
		}
	}
}
