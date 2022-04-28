use std::path::Path;
use libtransact::*;


fn main()
{
	let s = "

	      type, client, tx, amount
	   deposit,      1,  1,    1.0
	   deposit,      2,  2,    2.0
	   deposit,      1,  3,    2.0
	withdrawal,      1,  4,    1.5
	withdrawal,      2,  5,    3.0

	";


	let src = "tests/simple.csv";

	let csv = ParseCsv::from( s );
	// let csv = ParseCsv::try_from( Path::new(src) ).unwrap();

	for line in csv
	{
		let line = line.unwrap();
		println!( "{line:?}" );
	}
}
