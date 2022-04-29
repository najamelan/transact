#![ allow(missing_docs) ]

use crate::{ import::*, transaction::* };

/// The error type for errors happening in libtransact.
//
#[ allow(variant_size_differences) ]
#[ derive( Debug ) ]
//
pub enum TransErr
{
	/// Cannot open input file.
	//
	InputFile
	{
		source: std::io::Error,
		path  : PathBuf,
	},

	/// The input contained a transaction line that is invalid.
	//
	DeserializeTransact
	{
		source: Option<csv::Error>,
	},

	/// Failed to export CSV.
	//
	SerializeClients
	{
		source: fmt::Error,
	},

	/// A deposit or withdrawal with a transaction id that already exists came in.
	/// The transaction will be ignored as invalid.
	//
	DuplicateTransact
	{
		trans: Transact,
	},

	/// The client account is frozen.
	/// The transaction will be ignored as invalid.
	//
	AccountLocked
	{
		trans: Transact,
	},

	/// Cannot withdraw/dispute with insufficient funds.
	/// The transaction will be ignored as invalid.
	//
	InsufficientFunds
	{
		trans: Transact,
	},

	/// Cannot withdraw/dispute/resolve/charge back from non-existing client.
	/// The transaction will be ignored as invalid.
	//
	NoClient
	{
		trans: Transact,
	},

	/// Cannot dispute/resolve/charge back from a different client than the original deposit.
	/// The transaction will be ignored as invalid.
	//
	WrongClient
	{
		trans: Transact,
	},

	/// Can only dispute a successful transaction, resolve/charge back a disputed transaction.
	/// The transaction will be ignored as invalid.
	//
	WrongTransState
	{
		trans: Transact,
	},

	/// Disputed transaction does not exist.
	/// The transaction will be ignored as invalid.
	//
	ReferNoneExisting
	{
		trans: Transact,
	},

	/// Disputed transaction must be a deposit.
	/// The transaction will be ignored as invalid.
	//
	ShouldBeDeposit
	{
		trans: Transact,
	},

	/// Disputed transaction must be a successful transaction.
	/// The transaction will be ignored as invalid.
	//
	DisputeFailedTransact
	{
		trans: Transact,
	},
}



impl std::error::Error for TransErr
{
	fn source(&self) -> Option< &(dyn std::error::Error + 'static) >
	{
		match &self
		{
			TransErr::InputFile          { source, .. } => Some(source),
			TransErr::SerializeClients   { source     } => Some(source),
			TransErr::DeserializeTransact{ source     } =>
			{
				match source
				{
					Some(s) => Some(s),
					None    => None,
				}
			}

			TransErr::DuplicateTransact     {..} => None,
			TransErr::AccountLocked         {..} => None,
			TransErr::InsufficientFunds     {..} => None,
			TransErr::NoClient              {..} => None,
			TransErr::WrongClient           {..} => None,
			TransErr::WrongTransState       {..} => None,
			TransErr::ReferNoneExisting     {..} => None,
			TransErr::ShouldBeDeposit       {..} => None,
			TransErr::DisputeFailedTransact {..} => None,
		}
	}
}


impl std::fmt::Display for TransErr
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		let no_effect = "\nThis transaction has been ignored and not data was modified.";

		match &self
		{
			TransErr::InputFile{ source, path } =>

				writeln!( f, "Error: Could not open the supplied input file ({}): {source}", path.to_string_lossy() ),

			TransErr::SerializeClients{ source } =>

				writeln!( f, "Error: Failed to serialize client information: {source}" ),

			TransErr::DeserializeTransact{source} =>
			{
				writeln!( f, "Error: A line of input could not be deserialized into a valid transaction. {no_effect}" )?;

				if let Some(s) = source
				{
					return writeln!( f, "Underlying error: {s}" );
				}

				Ok(())
			}

			TransErr::DuplicateTransact{trans} =>

				writeln!( f, "Error: A duplicate transaction id occurred in your data: {trans:?}. {no_effect}" ),

			TransErr::AccountLocked{trans} =>

				writeln!( f, "Error: The client account is locked: {trans:?}. {no_effect}" ),

			TransErr::InsufficientFunds{trans} =>

				writeln!( f, "Error: Cannot withdraw/dispute with insufficient funds: {trans:?}. {no_effect}" ),

			TransErr::NoClient{trans} =>

				writeln!( f, "Error: Cannot withdraw/dispute/resolve/charge back from non-existing client: {trans:?}. {no_effect}" ),

			TransErr::WrongClient{trans} =>

				writeln!( f, "Error: Cannot dispute/resolve/charge back from a different client than the original deposit: {trans:?}. {no_effect}" ),

			TransErr::WrongTransState{trans} =>

				writeln!( f, "Error: Can only dispute a successful transaction, resolve/charge back a disputed transaction: {trans:?}. {no_effect}" ),

			TransErr::ReferNoneExisting{trans} =>

				writeln!( f, "Error: Cannot dispute/resolve/charge back a non existing transaction: {trans:?}. {no_effect}" ),

			TransErr::ShouldBeDeposit{trans} =>

				writeln!( f, "Error: Disputed transaction must be a deposit: {trans:?}. {no_effect}" ),

			TransErr::DisputeFailedTransact{trans} =>

				writeln!( f, "Error: Disputed transaction must be a successful transaction: {trans:?}. {no_effect}" ),
		}
	}
}


