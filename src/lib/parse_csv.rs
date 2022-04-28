use crate::{ import::*, transaction::*, TransErr };


/// A csv source for transactions. The format is as follows:
///
/// `
///       type, client, tx, amount
///    deposit,      1,  1,    1.0
///    deposit,      2,  2,    2.0
///    deposit,      1,  3,    2.0
/// withdrawal,      1,  4,    1.5
/// withdrawal,      2,  5,    3.0
/// `
///
/// ParseCsv will open the file when constructed and keep it open until dropped.
//
pub struct ParseCsv<T>
{
	source: csv::ByteRecordsIntoIter<T>,
	header: csv::ByteRecord,
}



impl<T: std::io::Read > ParseCsv<T>
{
	/// Create a new file based source for Csv data.
	//
	pub fn new( reader: T ) -> Self
	{
		let source = csv::ReaderBuilder::new().trim(csv::Trim::All).from_reader( reader ).into_byte_records();
		let header = csv::ByteRecord::from( vec![ "type", "client", "tx", "amount" ] );

		Self{ source, header }
	}
}


impl<T: std::io::Read> Iterator for ParseCsv<T>
{
	type Item = Result<Transact, TransErr>;

	fn next( &mut self ) -> Option<Self::Item>
	{
		if let Some(result) = self.source.next()
		{
			let cr = match result
			{
				Ok (r) => r,
				Err(e) => return Some(Err(TransErr::DeserializeTransact{ source: Some(e) } )),
			};

			match cr.deserialize::<CsvRecord<'_>>( Some(&self.header) )
			{
				Ok (r) => return Some( Transact::try_from(r) ),
				Err(e) => return Some(Err(TransErr::DeserializeTransact{ source: Some(e) } )),
			}
		}

		None
	}
}



impl<T> fmt::Debug for ParseCsv<T>
{
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> fmt::Result
	{
		Ok(())
	}
}




impl From< &'static str > for ParseCsv< &[u8] >
{
	fn from( s: &'static str  ) -> ParseCsv< &[u8] >
	{
		ParseCsv::new( s.trim().as_bytes() )
	}
}


// If you were hoping for:
// impl<P: AsRef<Path>> TryFrom<P> for ParseCsv
//
// It's not going to happen: https://github.com/rust-lang/rust/issues/50133
//
impl TryFrom< &Path > for ParseCsv<File>
{
	type Error = TransErr;

	fn try_from( p: &Path ) -> Result<ParseCsv<File>, TransErr>
	{
		let file = std::fs::File::open( p )

			.map_err( |e| TransErr::InputFile
			{
				source: e,
				path: p.to_path_buf()
			})?
		;

		Ok( ParseCsv::new( file ) )
	}
}