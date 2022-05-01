#![cfg_attr(nightly, feature(doc_cfg))]
#![ doc = include_str!("../../README.md") ]
#![doc(html_root_url = "https://docs.rs/transact")]
#![forbid(unsafe_code)]
#![allow(clippy::suspicious_else_formatting)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

mod bank;
mod client;
mod csv_export;
mod csv_parse;
mod trans_err;
mod transaction;

pub use bank::*;
pub use client::*;
pub use csv_export::*;
pub use csv_parse::*;
pub use trans_err::*;
pub use transaction::*;

// External dependencies
//
mod import {
    pub(crate) use {
        bigdecimal::{BigDecimal, Signed},
        serde::Deserialize,
        std::{
            borrow::Cow,
            collections::HashMap,
            fmt,
            fmt::Write,
            fs::File,
            path::{Path, PathBuf},
        },
    };
}
