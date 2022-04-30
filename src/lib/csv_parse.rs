use crate::{ import::*, transaction::*, TransErr };
use once_cell::unsync::Lazy;


/// A csv source for transactions. The format is as follows:
///
/// ```csv
///       type, client, tx, amount
///    deposit,      1,  1,    1.0
///    deposit,      2,  2,    2.0
///    deposit,      1,  3,    2.0
/// withdrawal,      1,  4,    1.5
/// withdrawal,      2,  5,    3.0
/// ```
///
/// CsvParse will open the file when constructed and keep it open until dropped.
//
pub struct CsvParse<T>
{
	source: csv::StringRecordsIntoIter<T>,
}



impl<T: std::io::Read > CsvParse<T>
{
	/// Create a new file based source for Csv data.
	//
	pub fn new( reader: T ) -> Self
	{
		let source = csv::ReaderBuilder::new()

			.trim( csv::Trim::All )
			.from_reader( reader )
			.into_records()
		;

		Self{ source }
	}
}


impl<T: std::io::Read> Iterator for CsvParse<T>
{
	type Item = Result<Transact, TransErr>;

	fn next( &mut self ) -> Option<Self::Item>
	{
		let header = Lazy::new( || csv::StringRecord::from( vec![ "type", "client", "tx", "amount" ] ) );

		if let Some(result) = self.source.next()
		{
			let cr = match result
			{
				Ok (r) => r,
				Err(e) => return Some(Err(TransErr::DeserializeTransact{ source: Some(e) } )),
			};

			match cr.deserialize::<CsvRecord<'_>>( Some(&header) )
			{
				Ok (r) => return Some( Transact::try_from(r) ),
				Err(e) => return Some(Err(TransErr::DeserializeTransact{ source: Some(e) } )),
			}
		}

		None
	}
}



impl<T> fmt::Debug for CsvParse<T>
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> fmt::Result
	{
		write!( f, "CsvParse" )
	}
}




impl From< &'static str > for CsvParse< &[u8] >
{
	fn from( s: &'static str  ) -> CsvParse< &[u8] >
	{
		CsvParse::new( s.trim().as_bytes() )
	}
}


// If you were hoping for:
// impl<P: AsRef<Path>> TryFrom<P> for CsvParse
//
// It's not going to happen: https://github.com/rust-lang/rust/issues/50133
//
impl TryFrom< &Path > for CsvParse<File>
{
	type Error = TransErr;

	fn try_from( p: &Path ) -> Result<CsvParse<File>, TransErr>
	{
		let file = std::fs::File::open( p )

			.map_err( |e| TransErr::InputFile
			{
				source: e,
				path: p.to_path_buf()
			})?
		;

		Ok( CsvParse::new( file ) )
	}
}
