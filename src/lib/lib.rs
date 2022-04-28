#![ cfg_attr( nightly, feature(doc_cfg) ) ]
#![ doc = include_str!("../../README.md") ]

#![ doc    ( html_root_url = "https://docs.rs/transact" ) ]
#![ forbid ( unsafe_code                                ) ]
#![ allow  ( clippy::suspicious_else_formatting         ) ]

#![ warn
(
	anonymous_parameters          ,
	missing_copy_implementations  ,
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	single_use_lifetimes          ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unreachable_pub               ,
	unused_extern_crates          ,
	unused_qualifications         ,
	variant_size_differences      ,
)]

mod error;
mod transaction;
mod bank;
mod client;
mod parse_csv;

pub use bank        ::*;
pub use client      ::*;
pub use parse_csv   ::*;
pub use error       ::*;
pub use transaction ::*;


// External dependencies
//
mod import
{
	pub(crate) use
	{
		std :: { path::{ Path, PathBuf }, fs::File, fmt, collections::HashMap } ,
		serde:: { Serialize, Deserialize },
	};
}



// struct ByteRecord;

// impl<'a, T: AsRef<[u8]>> From<&'a [T]> for ByteRecord
// {
// 	fn from( _s: &'a [T] ) -> Self
// 	{
// 		Self
// 	}
// }

// fn works<'a, T: AsRef<[u8]>>( _s: &'a [T] ) -> ByteRecord
// {
// 	ByteRecord
// }

// fn works2<'a, T: AsRef<[u8]>>( s: &'a [T] ) -> ByteRecord
// {
// 	ByteRecord::from(s)
// }

// fn main()
// {
// 	works2( &["string"] );
//    //ByteRecord::from( &["string"] );
// }
