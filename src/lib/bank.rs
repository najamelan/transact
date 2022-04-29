use crate::{ import::*, transaction::*, client::Client, TransErr };



/// The central unit that processes transactions and keeps client balances.
//
#[ derive( Debug, Default) ]
//
pub struct Bank
{
	clients: HashMap< u16, Client >,

	// This mocks a DB. Since we need access to past transactions for dispute, resolve and chargeback,
	// there isn't much of a choice here.
	//
	db: HashMap< u32, Transact >,

	errors: Vec<TransErr>,
}


#[ derive( Debug, PartialEq, Eq ) ]
//
enum Resolution { Resolve, ChargeBack }



impl Bank
{
	/// Create a new bank.
	//
	pub fn new() -> Self
	{
		Self
		{
			db     : HashMap::new(),
			clients: HashMap::new(),
			errors : Vec::new(),
		}
	}


	/// Get all the clients and their balances.
	//
	pub fn clients( &self ) -> &HashMap<u16, Client>
	{
		&self.clients
	}


	/// Get all the clients (mutable) and their balances.
	//
	pub fn clients_mut( &mut self ) -> &mut HashMap<u16, Client>
	{
		&mut self.clients
	}


	/// Process a list of transactions. Will return a list of all the errors that happened
	/// during processing. Transactions that cause an error will not affect any balances.
	//
	pub fn run( &mut self, source: impl Iterator<Item=Result<Transact, TransErr>> ) -> &[TransErr]
	{
		for result in source
		{
			let trans = match result
			{
				Ok(t) => t,

				Err(e) =>
				{
					self.errors.push( e );
					continue;
				}
			};


			// Get the client for this transaction. If it is a deposit we can create them, otherwise they
			// should already exist.
			//
			let client =

				if matches!( trans.ttype, TransType::Deposit(_) )
				{
					self.clients

						.entry( trans.client )
						.or_insert_with( || Client::new( trans.client ) )
				}

				else
				{
					match self.clients.get_mut( &trans.client )
					{
						Some(c) => c,

						None =>
						{
							self.errors.push( TransErr::NoClient{ trans } );
							continue;
						}
					}
				}
			;



			// client account should not be locked. No operation shall happen on a locked account.
			//
			if client.is_locked()
			{
				self.errors.push( TransErr::AccountLocked{ trans } );
				continue;
			}


			// Handle each type of transaction.
			//
			match trans.ttype
			{
				TransType::Deposit (amount) => Self::deposit    ( &mut self.db, &mut self.errors, client, trans, amount                 ),
				TransType::WithDraw(amount) => Self::withdraw   ( &mut self.db, &mut self.errors, client, trans, amount                 ),
				TransType::Dispute          => Self::dispute    ( &mut self.db, &mut self.errors, client, trans                         ),
				TransType::Resolve          => Self::resolution ( &mut self.db, &mut self.errors, client, trans, Resolution::Resolve    ),
				TransType::ChargeBack       => Self::resolution ( &mut self.db, &mut self.errors, client, trans, Resolution::ChargeBack ),
			}
		}

		&self.errors
	}


	/// Effectuate a deposit.
	///
	/// A deposit is a credit to the client's asset account, meaning it should increase the available and
	/// total funds of the client account.
	///
	/// Constraints:
	///
	/// - the transaction id should not exist.
	///
	//
	fn deposit( db: &mut HashMap<u32, Transact>, errors: &mut Vec<TransErr>, client: &mut Client, mut trans: Transact, amount: f64 )
	{
		// the transaction id should not exist
		//
		if db.get( &trans.id ).is_some()
		{
			errors.push( TransErr::DuplicateTransact{ trans } );
			return;
		}


		match client.update_balance( client.available() + amount, client.held() )
		{
			Ok(_) =>
			{
				trans.state = TransState::Success;
				db.insert( trans.id, trans );
			}

			Err(e) => errors.push( (trans, e).into() ),
		}
	}



	/// Effectuate a withdrawal.
	///
	/// A withdraw is a debit to the client's asset account, meaning it should decrease the available and
	/// total funds of the client account
	///
	/// If a client does not have sufficient available funds the withdrawal should fail and the total amount
	/// of funds should not change
	///
	/// Constraints:
	///
	/// - transaction id should not exist yet.
	/// - client.available >= amount.
	///
	//
	fn withdraw( db: &mut HashMap<u32, Transact>, errors: &mut Vec<TransErr>, client: &mut Client, mut trans: Transact, amount: f64 )
	{
		// transaction id should not exist yet.
		//
		if db.get( &trans.id ).is_some()
		{
			errors.push( TransErr::DuplicateTransact{ trans } );
			return;
		}


		// client.available should be >= amount
		//
		if  client.available() < amount
		{
			errors.push( TransErr::InsufficientFunds{ trans } );
			return;
		}


		match client.update_balance( client.available() - amount, client.held() )
		{
			Ok(_) =>
			{
				trans.state = TransState::Success;
				db.insert( trans.id, trans );
			}

			Err(e) => errors.push( (trans, e).into() ),
		}
	}



	/// Process dispute.
	///
	/// A dispute represents a client's claim that a transaction was erroneous and should be reversed.
	/// The transaction shouldn't be reversed yet but the associated funds should be held. This means
	/// that the clients available funds should decrease by the amount disputed, their held funds should
	/// increase by the amount disputed, while their total funds should remain the same.
	///
	/// Notice that a dispute does not state the amount disputed. Instead a dispute references the
	/// transaction that is disputed by ID. If the tx specified by the dispute doesn't exist you can ignore it
	/// and assume this is an error on our partners side.
	///
	/// Note that the spec do not mention what to do in case the available funds are insufficient.
	/// I shall consider that we cannot re-imburse funds that the client has already spent, thus the dispute
	/// will be ignored.
	///
	/// The specs do mention this: For example, a malicious actor may try to deposit fiat funds,
	/// purchase and withdraw BTC, and then reverse their fiat deposit.
	///
	/// I assume that refers to this situation.
	///
	/// Constraints:
	///
	/// - transaction should exist
	/// - transaction should be a deposit
	/// - transaction should be successful
	/// - client should equal client of disputed transaction.
	/// - client.available >= disputed amount
	//
	fn dispute( db: &mut HashMap<u32, Transact>, errors: &mut Vec<TransErr>, client: &mut Client, trans: Transact )
	{
		// transaction should exist
		//
		let old_trans = match db.get_mut( &trans.id )
		{
			Some(t) => t,

			None =>
			{
				errors.push( TransErr::ReferNoneExisting{ trans } );
				return;
			}
		};


		// client should equal client of disputed transaction.
		//
		if client.id() != old_trans.client
		{
			errors.push( TransErr::WrongClient{ trans } );
			return;
		}


		// transaction should be a deposit
		//
		let amount = match old_trans.ttype
		{
			TransType::Deposit(a) => a,

			_ =>
			{
				errors.push( TransErr::ShouldBeDeposit{ trans } );
				return;
			}
		};


		// deposit should be in successful state
		//
		if old_trans.state != TransState::Success
		{
			errors.push( TransErr::WrongTransState{ trans } );
			return;
		}


		// client.available should be >= disputed amount
		// If the client has already consumed the funds, they cannot dispute the deposit.
		//
		if client.available() < amount
		{
			errors.push( TransErr::InsufficientFunds{ trans } );
			return;
		}


		match client.update_balance( client.available() - amount, client.held() + amount )
		{
			Ok (_) => old_trans.state = TransState::Disputed ,
			Err(e) => errors.push( (trans, e).into() )       ,
		}
	}



	/// Process a resolve.
	///
	/// A resolve represents a resolution to a dispute, releasing the associated held funds. Funds that
	/// were previously disputed are no longer disputed. This means that the clients held funds should
	/// decrease by the amount no longer disputed, their available funds should increase by the
	/// amount no longer disputed, and their total funds should remain the same.
	///
	///
	/// Process a charge back.
	///
	/// A chargeback is the final state of a dispute and represents the client reversing a transaction.
	/// Funds that were held have now been withdrawn. This means that the clients held funds and
	/// total funds should decrease by the amount previously disputed. If a chargeback occurs the
	/// client's account should be immediately frozen.
	///
	/// Like a dispute and a resolve a chargeback refers to the transaction by ID (tx) and does not
	/// specify an amount. Like a resolve, if the tx specified doesn't exist, or the tx isn't under dispute,
	/// you can ignore chargeback and assume this is an error on our partner's side.
	///
	/// Constraints:
	///
	/// - transaction should exist
	/// - transaction should be disputed
	/// - client should equal client of disputed transaction.
	/// - client.held >= disputed amount
	//
	fn resolution( db: &mut HashMap<u32, Transact>, errors: &mut Vec<TransErr>, client: &mut Client, trans: Transact, action: Resolution )
	{
		// transaction should exist
		//
		let old_trans = match db.get_mut( &trans.id )
		{
			Some(t) => t,

			None =>
			{
				errors.push( TransErr::ReferNoneExisting{ trans } );
				return;
			}
		};


		// client should equal client of disputed transaction.
		//
		if client.id() != old_trans.client
		{
			errors.push( TransErr::WrongClient{ trans } );
			return;
		}


		// transaction should be a deposit
		//
		let amount = match old_trans.ttype
		{
			TransType::Deposit(a) => a,

			_ =>
			{
				errors.push( TransErr::ShouldBeDeposit{ trans } );
				return;
			}
		};


		// deposit should be in disputed state
		//
		if old_trans.state != TransState::Disputed
		{
			errors.push( TransErr::WrongTransState{ trans } );
			return;
		}


		// client.held should be >= disputed amount
		//
		// debug_assert because this should be impossible to hit, as we only allow resolving/chargback of disputed
		// transactions and when we dispute, we move the funds to held.
		// If there is not enough available funds in at the time of the dispute, the dispute gets
		// rejected, which will be caught above because the transaction state will not be disputed.
		//
		if client.held() < amount
		{
			debug_assert!( client.held() >= amount );
			errors.push( TransErr::InsufficientFunds{ trans } );
			return;
		}


		match action
		{
			Resolution::Resolve =>
			{
				match client.update_balance( client.available() + amount, client.held() - amount )
				{
					Ok (_) => old_trans.state = TransState::Success ,
					Err(e) => errors.push( (trans, e).into() )      ,
				}
			}

			Resolution::ChargeBack =>
			{
				match client.update_balance( client.available(), client.held() - amount )
				{
					Ok (_) =>
					{
						old_trans.state = TransState::ChargedBack;
						client.lock();
					}

					Err(e) => errors.push( (trans, e).into() ),
				}
			}
		}
	}
}



/// Some basic sanity tests. More complex scenarios will be tested in the integration tests.
//
#[ cfg(test) ]
//
mod test
{
	use crate::{ *, TransType::* };
	use pretty_assertions::{ assert_eq };


	fn locked_client() -> Bank
	{
		let mut bank = Bank::new();

		let trs: Vec<Result<_, TransErr>> = vec![ Ok( Transact::new( Deposit(3.2), 1, 1 ) ) ];

		bank.run( trs.into_iter() );
		bank.clients.get_mut(&1).unwrap().lock();

		bank
	}



	#[test] fn test_deposit()
	{
		let mut bank = Bank::new();

		let trs: Vec<Result<_, TransErr>> = vec!
		[
			Ok( Transact::new( Deposit(3.2), 1, 1 ) ),
			Ok( Transact::new( Deposit(2.3), 1, 2 ) ),
		];

		let errs   = bank.run( trs.into_iter() );
			assert_eq!( errs.len(), 0 );

		let client = bank.clients.get(&1).unwrap();
			assert_eq!( client.available(), 5.5 );
			assert_eq!( client.held()     , 0.0 );
			assert_eq!( client.total()  , 5.5 );
	}



	#[test] fn test_deposit_locked()
	{
		let mut bank = locked_client();

		let trs: Vec<Result<_, TransErr>> = vec!
		[
			Ok( Transact::new( Deposit(3.2), 1, 1 ) ),
		];

		let errs = bank.run( trs.into_iter() );
			assert_eq!( errs.len(), 1 );
			assert!(matches!( errs[0], TransErr::AccountLocked{..} ));
	}


	#[test] fn test_withdrawal()
	{
		let mut bank = Bank::new();

		let trs: Vec<Result<_, TransErr>> = vec!
		[
			Ok( Transact::new( Deposit (3.0), 1, 1 ) ),
			Ok( Transact::new( WithDraw(2.0), 1, 2 ) ),
		];

		let errs   = bank.run( trs.into_iter() );
			assert_eq!( errs.len(), 0 );

		let client = bank.clients.get(&1).unwrap();
			assert_eq!( client.available(), 1.0 );
			assert_eq!( client.held()     , 0.0 );
			assert_eq!( client.total()  , 1.0 );
	}


	#[test] fn test_withdrawal_locked()
	{
		let mut bank = locked_client();

		let trs: Vec<Result<_, TransErr>> = vec!
		[
			Ok( Transact::new( WithDraw(2.0), 1, 2 ) ),
		];

		let errs = bank.run( trs.into_iter() );
			assert_eq!( errs.len(), 1 );
			assert!(matches!( errs[0], TransErr::AccountLocked{..} ));
	}


	#[test] fn test_withdrawal_no_client()
	{
		let mut bank = Bank::new();

		let trs: Vec<Result<_, TransErr>> = vec!
		[
			Ok( Transact::new( WithDraw(2.0), 1, 2 ) ),
		];

		let errs = bank.run( trs.into_iter() );
			assert_eq!( errs.len(), 1 );
			assert!(matches!( errs[0], TransErr::NoClient{..} ));
	}


	#[test] fn test_withdrawal_no_funds()
	{
		let mut bank = Bank::new();

		let trs: Vec<Result<_, TransErr>> = vec!
		[
			Ok( Transact::new( Deposit (3.0), 1, 1 ) ),
			Ok( Transact::new( WithDraw(4.0), 1, 2 ) ),
		];

		let errs = bank.run( trs.into_iter() );
			assert_eq!( errs.len(), 1 );
			assert!(matches!( errs[0], TransErr::InsufficientFunds{..} ));
	}
}
