#![ allow(missing_docs) ]

use crate::{ import::*, * };


/// Different things that can go wrong in deserialization after csv has correctly
/// deserialized. This means the types were correct, but values are not compatible
/// with a
//
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
//
pub enum DeserTransactKind
{
	/// The csv file holds a negative amount for deposit or withdrawal.
	//
	AmountNegative,

	/// The transaction type is unknown. Only "deposit", "withdrawal", "dispute", "resolve" and chargeback
	/// are supported.
	//
	UnknownTransType,
}

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

	/// The input contained a transaction line that is invalid. These are errors from the csv crate.
	//
	DeserializeCsv
	{
		source: csv::Error,
	},

	/// The input contained a transaction line that is invalid.
	//
	DeserializeTransact
	{
		kind  : DeserTransactKind,
		record: CsvRecord<'static>,
	},

	/// The header could not be deserialized. Most likely it's not valid utf8.
	//
	DeserializeHeader
	{
		source: csv::Error,
	},

	/// The Csv file did not contain a valid header.
	//
	NoHeader,

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

	/// Cannot dispute/resolve/chargeback a non existing transaction.
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
}



impl std::error::Error for TransErr
{
	fn source(&self) -> Option< &(dyn std::error::Error + 'static) >
	{
		match &self
		{
			TransErr::InputFile          { source, .. } => Some(source),
			TransErr::SerializeClients   { source     } => Some(source),
			TransErr::DeserializeHeader  { source     } => Some(source),
			TransErr::DeserializeCsv     { source     } => Some(source),

			TransErr::DeserializeTransact{..} => None,
			TransErr::DuplicateTransact  {..} => None,
			TransErr::AccountLocked      {..} => None,
			TransErr::InsufficientFunds  {..} => None,
			TransErr::NoClient           {..} => None,
			TransErr::WrongClient        {..} => None,
			TransErr::WrongTransState    {..} => None,
			TransErr::ReferNoneExisting  {..} => None,
			TransErr::ShouldBeDeposit    {..} => None,
			TransErr::NoHeader                => None,
		}
	}
}


impl std::fmt::Display for TransErr
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		let no_effect = "\nThis transaction has been ignored and no data was modified by it.";

		match &self
		{
			TransErr::InputFile{ source, path } =>

				writeln!( f, "\nError: Could not open the supplied input file ({}): {source}", path.to_string_lossy() ),

			TransErr::SerializeClients{ source } =>

				writeln!( f, "\nError: Failed to serialize client information: {source}" ),

			TransErr::DeserializeHeader{ source } =>

				writeln!( f, "\nError: The header could not be deserialized. Underlying error: {source}" ),

			TransErr::DeserializeCsv{source} =>

				write!( f, "\nError: A line of input could not be deserialized into a valid transaction: {source}. {no_effect}" ),

			TransErr::DeserializeTransact{ kind, record } =>

				writeln!( f, "\nError: Could not use deserialized data to construct a valid transaction: {kind:?}, {record}. {no_effect}" ),

			TransErr::DuplicateTransact{trans} =>

				writeln!( f, "\nError: A duplicate transaction id occurred in your data: {trans}. {no_effect}" ),

			TransErr::AccountLocked{trans} =>

				writeln!( f, "\nError: The client account is locked: {trans}. {no_effect}" ),

			TransErr::InsufficientFunds{trans} =>

				writeln!( f, "\nError: Cannot withdraw/dispute with insufficient funds: {trans}. {no_effect}" ),

			TransErr::NoClient{trans} =>

				writeln!( f, "\nError: Cannot withdraw/dispute/resolve/charge back from non-existing client: {trans}. {no_effect}" ),

			TransErr::WrongClient{trans} =>

				writeln!( f, "\nError: Cannot dispute/resolve/charge back from a different client than the original deposit: {trans}. {no_effect}" ),

			TransErr::WrongTransState{trans} =>

				writeln!( f, "\nError: Can only dispute a successful transaction, resolve/charge back a disputed transaction: {trans}. {no_effect}" ),

			TransErr::ReferNoneExisting{trans} =>

				writeln!( f, "\nError: Cannot dispute/resolve/charge back a non existing transaction: {trans}. {no_effect}" ),

			TransErr::ShouldBeDeposit{trans} =>

				writeln!( f, "\nError: Disputed transaction must be a deposit: {trans}. {no_effect}" ),

			TransErr::NoHeader =>

				writeln!( f, "\nError: Only CSV files with a valid header are supported. For a valid header the first line should be: \"type, client, tx, amount\"" ),
		}
	}
}

