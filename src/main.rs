use std::{ path::Path, process::exit };
use libtransact::*;


fn main() -> Result<(), Box<dyn std::error::Error> >
{
	let args: Vec<String> = std::env::args().skip(1).collect();

	if args.len() != 1
	{
		eprintln!( "Error: The transact takes exactly one argument. A path to a CSV file with transaction. Got {} arguments.", args.len() );
		exit( 1 );
	}

	let csv = ParseCsv::try_from( Path::new(&args[0]) )?;

	let mut bank = Bank::new();

	let errors = bank.run( csv );
	// errors.iter().for_each( |e| eprint!( "{}", e ) );
	let num_err = errors.len() as i32;

	let out = CsvExport::export( bank.clients() )?;
	print!( "{}", out );

	exit( num_err );
}
