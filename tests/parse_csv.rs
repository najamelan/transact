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
//!   ✓ invalid utf in header reports error
//!   ✓ invalid utf in value causes just this transaction to be ignored
//!   ✓ file with missing header reports error
//!   - dispute, resolve, charge back with amount.
//!   - deposit/withdraw without amount.
//!   - non numeric values.
//
mod common;

use
{
	common            :: *          ,
	libtransact       :: *          ,
	pretty_assertions :: assert_eq  ,
	std               :: path::Path ,
};



#[test] fn file_input() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/simple.csv") )?;
	let mut bank = Bank::new();


	let err = bank.process( parser );

		assert_eq!( err.len(), 0, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), dec("1.5") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.5") );


	let client = bank.clients().get(&2).unwrap();

		assert_eq!( client.available(), dec("1.9") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.9") );


	Ok(())
}



#[test] fn empty_leading() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/empty_leading.csv") )?;
	let mut bank = Bank::new();


	let err = bank.process( parser );

		assert_eq!( err.len(), 0, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), dec("1.5") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.5") );


	let client = bank.clients().get(&2).unwrap();

		assert_eq!( client.available(), dec("1.9") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.9") );


	Ok(())
}



#[test] fn empty_trailing() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/empty_trailing.csv") )?;
	let mut bank = Bank::new();


	let err = bank.process( parser );

		assert_eq!( err.len(), 0, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), dec("1.5") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.5") );


	let client = bank.clients().get(&2).unwrap();

		assert_eq!( client.available(), dec("1.9") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.9") );


	Ok(())
}



#[test] fn empty_middle() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/empty_middle.csv") )?;
	let mut bank = Bank::new();


	let err = bank.process( parser );

		assert_eq!( err.len(), 0, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), dec("1.5") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.5") );


	let client = bank.clients().get(&2).unwrap();

		assert_eq!( client.available(), dec("1.9") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.9") );


	Ok(())
}



#[test] fn invalid_line() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/invalid_line.csv") )?;
	let mut bank = Bank::new();


	let err = bank.process( parser );

		assert_eq!( err.len(), 2, "{err:?}"                               );
		assert!   ( matches!( err[0], TransErr::DeserializeTransact{..} ) );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), dec("1.5") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.5") );


	Ok(())
}


// Files without header are not accepted.
//
#[test] fn no_headers()
{
	let parser = CsvParse::try_from( Path::new("tests/data/no_headers.csv") );

	assert!( matches!( parser, Err(TransErr::NoHeader) ) );
}


// invalid utf in header is reported
//
#[test] fn invalid_utf8_in_header()
{
	let parser = CsvParse::try_from( Path::new("tests/data/invalid_utf8_in_header.csv") );

	assert!( matches!(
		parser,
		Err( TransErr::DeserializeHeader{ source } ) if matches!( source.kind(), &csv::ErrorKind::Utf8{..} )
	));
}


// invalid utf in value causes just this transaction to be ignored
//
#[test] fn invalid_utf8_in_value() -> DynResult
{
	let parser   = CsvParse::try_from( Path::new("tests/data/invalid_utf8_in_value.csv") )?;
	let mut bank = Bank::new();


	let err = bank.process( parser );

		assert_eq!( err.len(), 1, "{err:?}" );


	let client = bank.clients().get(&1).unwrap();

		assert_eq!( client.available(), dec("0.5") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("0.5") );


	let client = bank.clients().get(&2).unwrap();

		assert_eq!( client.available(), dec("1.9") );
		assert_eq!( client.held()     , dec("0.0") );
		assert_eq!( client.total()    , dec("1.9") );


	Ok(())
}
