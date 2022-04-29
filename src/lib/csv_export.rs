use crate::{ import::*, client::*, error::* };


/// Namespace for the export function. Takes care of exporting the client data to CSV.
//
#[ derive( Debug, Copy, Clone ) ]
//
pub struct CsvExport {}


impl CsvExport
{
	/// Export the client data to CSV.
	//
	pub fn export( clients: &HashMap< u16, Client > ) -> Result<String, TransErr>
	{
		let mut width = 12;
		let mut out = String::new();

		std::writeln!( out, "{:>width$}{:>width$}{:>width$}{:>width$}{:>width$}", "client,", "available,", "held,", "total,", "locked" )

			.map_err( |source| TransErr::SerializeClients{ source } )?
		;

		width -= 1;

		for (i, c) in clients
		{
			std::writeln!( out, "{:>width$},{:>width$},{:>width$},{:>width$},{:>width$}", i, c.available(), c.held(), c.total(), c.is_locked() )

				.map_err( |source| TransErr::SerializeClients{ source } )?
			;
		}



		Ok( out )
	}
}
