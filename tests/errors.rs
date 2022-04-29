//! This tests reading in csv data and processing it correctly with the Bank API.
//! Basic tests for deposit and withdrawal are in unit tests on src/lib/bank.rs.
//!
//! For the sake of the exercise, these tests are not exhaustive.
//!
//! Tested:
//!
//! - AccountLocked
//!   ✓ try deposit    on a locked account
//!   ✓ try withdraw   on a locked account
//!   ✓ try dispute    on a locked account
//!   ✓ try resolve    on a locked account
//!   ✓ try chargeback on a locked account
//!
//! - DuplicateTransact
//!
//!   ✓ with same id, try deposit deposit
//!   ✓ with same id, try deposit withdraw
//!   ✓ with same id, try deposit withdraw deposit
//!
//! - InsufficientFunds:
//!
//!   ✓ try to withdraw more than available
//!   ✓ try to dispute funds no longer available
//
use
{
	libtransact::*               ,
	pretty_assertions::assert_eq ,
};

// type DynResult<T = ()> = Result<T, Box< dyn std::error::Error + Send + Sync> >;

fn locked_client() -> Bank
{
	let mut bank = Bank::new();

	let trs: Vec<Result<_, TransErr>> = vec!
	[
		Ok( Transact::new( TransType::Deposit(3.2), 1, 1 ) ) ,
		Ok( Transact::new( TransType::Deposit(2.0), 1, 2 ) ) ,
		Ok( Transact::new( TransType::Dispute     , 1, 1 ) ) ,
	];


	let errs = bank.run( trs.into_iter() );

		assert!( errs.is_empty() );


	bank.clients_mut().get_mut(&1).unwrap().lock();
	bank
}



////////////////////
// Locked account //
////////////////////


// try deposit on a locked account
//
#[test] fn locked_deposit()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  3,    1.0

	";

	let parser   = CsvParse::from( input );
	let mut bank = locked_client();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );

		assert!( matches!( err[0],

			TransErr::AccountLocked { trans: Transact
			{
				ttype : TransType::Deposit(a) ,
				state : TransState::New       ,
				client: 1                     ,
				id    : 3                     ,

			}} if a == 1.0
		));

	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 2.0 );
		assert_eq!( client.held()     , 3.2 );
		assert_eq!( client.total()    , 5.2 );
}


// try withdrawal on a locked account
//
#[test] fn locked_withdraw()
{
	let input = "

		      type, client, tx, amount
		withdrawal,      1,  3,    1.0

	";

	let parser   = CsvParse::from( input );
	let mut bank = locked_client();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );

		assert!( matches!( err[0],

			TransErr::AccountLocked { trans: Transact
			{
				ttype : TransType::WithDraw(a) ,
				state : TransState::New        ,
				client: 1                      ,
				id    : 3                      ,

			}} if a == 1.0
		));

	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 2.0 );
		assert_eq!( client.held()     , 3.2 );
		assert_eq!( client.total()    , 5.2 );
}


// try dispute on a locked account
//
#[test] fn locked_dispute()
{
	let input = "

		      type, client, tx, amount
		   dispute,      1,  2,

	";

	let parser   = CsvParse::from( input );
	let mut bank = locked_client();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );

		assert!( matches!( err[0],

			TransErr::AccountLocked { trans: Transact
			{
				ttype : TransType::Dispute ,
				state : TransState::New    ,
				client: 1                  ,
				id    : 2                  ,
			}}
		));

	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 2.0 );
		assert_eq!( client.held()     , 3.2 );
		assert_eq!( client.total()    , 5.2 );
}


// try resolve on a locked account
//
#[test] fn locked_resolve()
{
	let input = "

		      type, client, tx, amount
		   resolve,      1,  1,

	";

	let parser   = CsvParse::from( input );
	let mut bank = locked_client();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );

		assert!( matches!( err[0],

			TransErr::AccountLocked { trans: Transact
			{
				ttype : TransType::Resolve ,
				state : TransState::New    ,
				client: 1                  ,
				id    : 1                  ,
			}}
		));

	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 2.0 );
		assert_eq!( client.held()     , 3.2 );
		assert_eq!( client.total()    , 5.2 );
}


// try charge back on a locked account
//
#[test] fn locked_charge_back()
{
	let input = "

		      type, client, tx, amount
		chargeback,      1,  1,

	";

	let parser   = CsvParse::from( input );
	let mut bank = locked_client();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );

		assert!( matches!( err[0],

			TransErr::AccountLocked { trans: Transact
			{
				ttype : TransType ::ChargeBack ,
				state : TransState::New        ,
				client: 1                      ,
				id    : 1                      ,
			}}
		));

	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 2.0 );
		assert_eq!( client.held()     , 3.2 );
		assert_eq!( client.total()    , 5.2 );
}



///////////////////////
// DuplicateTransact //
///////////////////////


// with same id, try deposit deposit
//
#[test] fn dup_deposit_deposit()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1,    1.0
		   deposit,      1,  1,    1.5

	";

	let parser   = CsvParse::from( input );
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );

		assert!( matches!( err[0],

			TransErr::DuplicateTransact { trans: Transact
			{
				ttype : TransType::Deposit(a) ,
				state : TransState::New       ,
				client: 1                     ,
				id    : 1                     ,

			}} if a == 1.5
		));

	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 1.0 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 1.0 );
}



// with same id, try deposit withdraw deposit
//
#[test] fn dup_deposit_withdraw_deposit()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1,    1.0
		withdrawal,      1,  2,    0.5
		   deposit,      1,  2,    2.0

	";

	let parser   = CsvParse::from( input );
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );

		assert!( matches!( err[0],

			TransErr::DuplicateTransact { trans: Transact
			{
				ttype : TransType::Deposit(a) ,
				state : TransState::New       ,
				client: 1                     ,
				id    : 2                     ,

			}} if a == 2.0
		));

	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 0.5 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 0.5 );
}



// with same id, try deposit withdraw
//
#[test] fn dup_deposit_withdraw()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1,    1.0
		withdrawal,      1,  1,    0.5

	";

	let parser   = CsvParse::from( input );
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );

		assert!( matches!( err[0],

			TransErr::DuplicateTransact { trans: Transact
			{
				ttype : TransType::WithDraw(a) ,
				state : TransState::New        ,
				client: 1                      ,
				id    : 1                      ,

			}} if a == 0.5
		));

	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 1.0 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 1.0 );
}




///////////////////////
// InsufficientFunds //
///////////////////////

// try to withdraw more than available
//
#[test] fn withdraw_too_much()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1,    1.0
		withdrawal,      1,  2,    1.5

	";

	let parser   = CsvParse::from( input );
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );

		assert!( matches!( err[0],

			TransErr::InsufficientFunds { trans: Transact
			{
				ttype : TransType::WithDraw(a) ,
				state : TransState::New        ,
				client: 1                      ,
				id    : 2                      ,

			}} if a == 1.5
		));

	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 1.0 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 1.0 );
}


// try to dispute funds no longer available
//
#[test] fn dispute_after_withdraw()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1,    1.0
		withdrawal,      1,  2,    0.5
		   dispute,      1,  1,

	";

	let parser   = CsvParse::from( input );
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1 );


	assert!( matches!( err[0],

		TransErr::InsufficientFunds { trans: Transact
		{
			ttype : TransType::Dispute ,
			state : TransState::New    ,
			client: 1                  ,
			id    : 1                  ,
		}}
	));


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 0.5 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 0.5 );
}

