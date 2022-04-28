//! This tests reading in csv data and processing it correctly with the Bank API.
//! Basic tests for deposit and withdrawal are in unit tests on src/lib/bank.rs.
//!
//! Tested:
//!
//! ✓ mix two clients
//! ✓ dispute
//! ✓ resolve dispute
//! ✓ chargeback dispute
//! ✓ take input from file
//! - run binary
//
use
{
	libtransact::*               ,
	pretty_assertions::assert_eq ,
	std::path::Path              ,
	std::process::Command        ,
};

type DynResult<T = ()> = Result<T, Box< dyn std::error::Error + Send + Sync> >;

#[test] fn test_2_clients()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1,    1.0
		   deposit,      2,  2,    2.0
		   deposit,      1,  3,    2.0
		withdrawal,      1,  4,    1.5
		withdrawal,      2,  5,    1.5

	";

	let parser   = ParseCsv::from( input );
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 0 );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 1.5 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()  , 1.5 );


	let client = bank.clients().get(&2).unwrap();

		assert_eq!( client.available(), 0.5 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 0.5 );

}


#[test] fn test_dispute()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1, 0.66
		   deposit,      1,  2, 0.3333
		   dispute,      1,  2,

	";

	let parser   = ParseCsv::from( input );
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 0 );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 0.66   );
		assert_eq!( client.held()     , 0.3333 );
		assert_eq!( client.total()    , 0.9933 );
}


#[test] fn test_resolve()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1, 0.66
		   deposit,      1,  2, 0.3333
		   dispute,      1,  2,
		   resolve,      1,  2,

	";

	let parser   = ParseCsv::from( input );
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 0, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 0.9933 );
		assert_eq!( client.held()     , 0.0    );
		assert_eq!( client.total()    , 0.9933 );
}


#[test] fn test_chargeback()
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1, 0.66
		   deposit,      1,  2, 0.3333
		   dispute,      1,  2,
		   chargeback,   1,  2,

	";

	let parser   = ParseCsv::from( input );
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 0, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 0.66 );
		assert_eq!( client.held()     , 0.0  );
		assert_eq!( client.total()    , 0.66 );
}


#[test] fn test_file_input() -> DynResult
{
	let parser   = ParseCsv::try_from( Path::new("tests/simple.csv") )?;
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 0, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 1.5 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 1.5 );


	let client = bank.clients().get(&2).unwrap();

		assert_eq!( client.available(), 1.9 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 1.9 );


	Ok(())
}


#[test] fn test_cli() -> DynResult
{
	let output = Command::new("cargo")

		.arg( "run" )
		.arg( "--"  )
		.arg( "tests/simple.csv"  )
		.output()?
	;

	assert_eq!( std::str::from_utf8(&output.stdout)?, "     client,  available,       held,      total,      locked
          1,        1.5,          0,        1.5,      false
          2,        1.9,          0,        1.9,      false
" );

	Ok(())
}
