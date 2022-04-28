use std::{ path::Path, io::Write };
use libtransact::*;


fn main() -> Result<(), Box<dyn std::error::Error> >
{
	// let s = "

	//       type, client, tx, amount
	//    deposit,      1,  1,    1.0
	//    deposit,      2,  2,    2.0
	//    deposit,      1,  3,    2.0
	// withdrawal,      1,  4,    1.5
	// withdrawal,      2,  5,    3.0

	// ";


	let src = "tests/simple.csv";

	// let csv = ParseCsv::from( s );
	let csv = ParseCsv::try_from( Path::new(src) ).unwrap();

	let mut bank = Bank::new();

	let errors = bank.run( csv );
	errors.iter().for_each( |e| eprint!( "{}", e ) );
	let num_err = errors.len() as i32;

	let out = CsvExport::export( bank.clients() )?;
	print!( "{}", out );

	std::process::exit( num_err );
}
