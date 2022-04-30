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
//! ✓ run binary
//
use
{
	libtransact::*               ,
	pretty_assertions::assert_eq ,
	std::process::Command        ,
};

type DynResult<T = ()> = Result<T, Box< dyn std::error::Error + Send + Sync> >;

#[test] fn two_clients() -> DynResult
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1,    1.0
		   deposit,      2,  2,    2.0
		   deposit,      1,  3,    2.0
		withdrawal,      1,  4,    1.5
		withdrawal,      2,  5,    1.5

	";

	let parser   = CsvParse::try_from( input )?;
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 0 );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 1.5 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 1.5 );


	let client = bank.clients().get(&2).unwrap();

		assert_eq!( client.available(), 0.5 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 0.5 );

	Ok(())
}


#[test] fn dispute() -> DynResult
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1, 0.66
		   deposit,      1,  2, 0.3333
		   dispute,      1,  2,

	";

	let parser   = CsvParse::try_from( input )?;
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 0 );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 0.66   );
		assert_eq!( client.held()     , 0.3333 );
		assert_eq!( client.total()    , 0.9933 );

	Ok(())
}


#[test] fn resolve() -> DynResult
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1, 0.66
		   deposit,      1,  2, 0.3333
		   dispute,      1,  2,
		   resolve,      1,  2,

	";

	let parser   = CsvParse::try_from( input )?;
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 0, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 0.9933 );
		assert_eq!( client.held()     , 0.0    );
		assert_eq!( client.total()    , 0.9933 );

	Ok(())
}


#[test] fn chargeback() -> DynResult
{
	let input = "

		      type, client, tx, amount
		   deposit,      1,  1, 0.66
		   deposit,      1,  2, 0.3333
		   dispute,      1,  2,
		   chargeback,   1,  2,

	";

	let parser   = CsvParse::try_from( input )?;
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 0, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 0.66 );
		assert_eq!( client.held()     , 0.0  );
		assert_eq!( client.total()    , 0.66 );

	Ok(())
}


#[test] fn cli() -> DynResult
{
	let output = Command::new("cargo")

		.arg( "run" )
		.arg( "--"  )
		.arg( "tests/data/simple.csv"  )
		.output()?
	;

	// Since order of the clients is not deterministic, we cannot test an exact outcome.
	//
	let out = std::str::from_utf8(&output.stdout)?;

	assert!( out.contains( "client,  available,       held,      total,      locked" ) );
	assert!( out.contains(      "1,        1.5,          0,        1.5,      false"  ) );
	assert!( out.contains(      "2,        1.9,          0,        1.9,      false"  ) );

	Ok(())
}
