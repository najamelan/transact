//! This tests reading in csv data and processing it correctly with the Bank API.
//! Basic tests for deposit and withdrawal are in unit tests on src/lib/bank.rs.
//!
//! Tested:
//!
//! ✓
//
use
{
	libtransact::*               ,
	pretty_assertions::assert_eq ,
	std::path::Path              ,
};

type DynResult<T = ()> = Result<T, Box< dyn std::error::Error + Send + Sync> >;



#[test] fn test_file_input() -> DynResult
{
	let parser   = ParseCsv::try_from( Path::new("tests/data/simple.csv") )?;
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



#[test] fn test_empty_leading() -> DynResult
{
	let parser   = ParseCsv::try_from( Path::new("tests/data/empty_leading.csv") )?;
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



#[test] fn test_empty_trailing() -> DynResult
{
	let parser   = ParseCsv::try_from( Path::new("tests/data/empty_trailing.csv") )?;
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



#[test] fn test_empty_middle() -> DynResult
{
	let parser   = ParseCsv::try_from( Path::new("tests/data/empty_middle.csv") )?;
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
