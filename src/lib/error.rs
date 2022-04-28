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
		let no_effect = "This transaction has been ignored and not data was modified.";

		match &self
		{
			TransErr::InputFile{ source, path } =>

				write!( f, "Error: Could not open the supplied input file ({}): {source}", path.to_string_lossy() ),

			TransErr::DeserializeTransact{source} =>
			{
				write!( f, "Error: A line of input could not be deserialized into a valid transaction. {no_effect}" )?;

				if let Some(s) = source
				{
					return write!( f, "Underlying error: {s}" );
				}

				Ok(())
			}

			TransErr::DuplicateTransact{trans} =>

				write!( f, "Error: A duplicate transaction id occurred in your data: {trans:?}. {no_effect}" ),

			TransErr::AccountLocked{trans} =>

				write!( f, "Error: The client account is locked: {trans:?}. {no_effect}" ),

			TransErr::InsufficientFunds{trans} =>

				write!( f, "Error: Cannot withdraw/dispute with insufficient funds: {trans:?}. {no_effect}" ),

			TransErr::NoClient{trans} =>

				write!( f, "Error: Cannot withdraw/dispute/resolve/charge back from non-existing client: {trans:?}. {no_effect}" ),

			TransErr::WrongClient{trans} =>

				write!( f, "Error: Cannot dispute/resolve/charge back from a different client than the original deposit: {trans:?}. {no_effect}" ),

			TransErr::WrongTransState{trans} =>

				write!( f, "Error: Can only dispute a successful transaction, resolve/charge back a disputed transaction: {trans:?}. {no_effect}" ),

			TransErr::ReferNoneExisting{trans} =>

				write!( f, "Error: Cannot dispute/resolve/charge back a non existing transaction: {trans:?}. {no_effect}" ),

			TransErr::ShouldBeDeposit{trans} =>

				write!( f, "Error: Disputed transaction must be a deposit: {trans:?}. {no_effect}" ),

			TransErr::DisputeFailedTransact{trans} =>

				write!( f, "Error: Disputed transaction must be a successful transaction: {trans:?}. {no_effect}" ),
		}
	}
}


