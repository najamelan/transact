//! This tests reading in csv data and processing it correctly with the Bank API.
//! Basic tests for deposit and withdrawal are in unit tests on src/lib/bank.rs.
//!
//! Tested:
//!
//! ✓ file input
//! ✓ file with leading empty lines
//! ✓ file with trailing empty lines
//! ✓ file with empty lines in the middle
//!
//! - Invalid input:
//!
//!   ✓ file with one invalid line.
//!   ✓ invalid utf in header is ignored
//!   ✓ invalid utf in value causes just this transaction to be ignored
//!   - dispute, resolve, charge back with amount.
//!   - deposit/withdraw without amount.
//!   - non numeric values.
//
use
{
	libtransact::*               ,
	pretty_assertions::assert_eq ,
	std::path::Path              ,
};

type DynResult<T = ()> = Result<T, Box< dyn std::error::Error + Send + Sync> >;



#[test] fn file_input() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/simple.csv") )?;
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



#[test] fn empty_leading() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/empty_leading.csv") )?;
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



#[test] fn empty_trailing() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/empty_trailing.csv") )?;
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



#[test] fn empty_middle() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/empty_middle.csv") )?;
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



#[test] fn invalid_line() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/invalid_line.csv") )?;
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 2, "{err:?}"                               );
		assert!   ( matches!( err[0], TransErr::DeserializeTransact{..} ) );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 1.5 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 1.5 );


	Ok(())
}


// invalid utf in header is ignored
//
#[test] fn invalid_utf8_in_header() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/invalid_utf8_in_header.csv") )?;
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


// invalid utf in value causes just this transaction to be ignored
//
#[test] fn invalid_utf8_in_value() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/invalid_utf8_in_value.csv") )?;
	let mut bank = Bank::new();


	let err = bank.run( parser );

		assert_eq!( err.len(), 1, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), 0.5 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 0.5 );


	let client = bank.clients().get(&2).unwrap();

		assert_eq!( client.available(), 1.9 );
		assert_eq!( client.held()     , 0.0 );
		assert_eq!( client.total()    , 1.9 );


	Ok(())
}
