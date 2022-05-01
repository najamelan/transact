use {bigdecimal::BigDecimal, std::str::FromStr};

pub type DynResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Shorthand for creating BigDecimal.
//
pub fn dec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}
