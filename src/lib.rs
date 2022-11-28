mod try_parse;
mod error;
mod option_types;
pub mod macros;

pub mod prelude {
    pub use crate::try_parse::TryParse;
    pub use crate::error::ArgParsingError;
    pub use crate::option_types::OptionReceptacle;
}

pub mod opt_types {
    pub use crate::option_types::{
        Counter, Flag, FlagCounter,
        GrabFirst, GrabLast, GrabAll
    };
}

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