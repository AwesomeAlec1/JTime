pub mod temporal;

mod duration;
mod instant;
mod local_time;
mod local_date_time;
mod local_date;

pub use duration::*;
pub use instant::*;
pub use local_date::*;
pub use local_date_time::*;
pub use local_time::*;

use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DateTimeError
{
    #[error("{0}")]
    Unspecified(Arc<str>),
    #[error("Unsupported temporal type")]
    UnsupportedTemporalType,
    #[error("Parse error at position {1}: {0}")]
    ParseError(Arc<str>, usize),
    #[error("{0}")]
    ZoneRulesError(Arc<str>),
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
