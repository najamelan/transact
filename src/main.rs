#![forbid(unsafe_code)]

use std::{ path::Path, process::exit };
use libtransact::*;



/// Simple CLI frontend which will collect the first argument as a filename. The file is assumed
/// to be transactions encoded in CSV with comma separated values.
///
/// Erroneous transactions will be skipped and errors will be printed to stderr with the status
/// code representing how many errors occurred.
//
fn main()
{
	// first argument is the process path.
	//
	let args: Vec<String> = std::env::args().skip(1).collect();

	if args.len() != 1
	{
		eprintln!( "Error: The transact takes exactly one argument. A path to a CSV file with transaction. Got {} arguments.", args.len() );
		exit( 1 );
	}

	let transactions = match CsvParse::try_from( Path::new(&args[0]) )
	{
		Ok(parser) => parser,

		Err(e) =>
		{
			eprintln!( "{e}" );
			exit(1);
		}
	};

	let mut bank = Bank::new();

	let errors = bank.process( transactions );

	// report errors on stderr.
	//
	errors.iter().for_each( |e| eprint!( "{}", e ) );

	let num_err = errors.len() as i32;

	// report results on stdout.
	//
	let out = match CsvExport::export( bank.clients() )
	{
		Ok(out) => out,

		Err(e) =>
		{
			eprintln!( "{e}" );
			exit( num_err + 1 );
		}
	};

	// happy path
	//
	print!( "{}", out );

	// report the number of errors in the status code.
	//
	exit( num_err );
}
