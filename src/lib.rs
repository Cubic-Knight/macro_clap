//! To use this crate, write `use macro_clap::*;` in your own crate
//! 
//! You can also use
//! ```rust
//! use macro_clap::prelude::*;
//! use macro_clap::cli_macro::*;
//! use macro_clap::opt_types::*;
//! ```
//! which is equivalent

mod try_parse;
mod error;
mod option_types;
mod macros;

/// Error types and traits
pub mod prelude {
    pub use crate::try_parse::TryParse;
    pub use crate::error::ArgParsingError;
    pub use crate::option_types::OptionReceptacle;
}

/// Option types
pub mod opt_types {
    pub use crate::option_types::{
        Counter, Flag, FlagCounter,
        GrabFirst, GrabLast, GrabAll
    };
}

/// All necessary macros
pub mod cli_macro {
    pub use crate::{
        cli, arg, maybe,
        branch, opt, collect,
        impl_type, usage
    };
}

pub use crate::try_parse::TryParse;
pub use crate::error::ArgParsingError;
pub use crate::option_types::{
    Counter, Flag, FlagCounter,
    GrabFirst, GrabLast, GrabAll,
    OptionReceptacle
};