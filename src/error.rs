#[derive(Debug)]
pub enum ArgParsingError {
    NoProgramName, // 

    Introduction(&'static str),
    Help,

    // Unused arguments/options
    UnknownArgument(String),
    UnknownOption(String, Option<String>),

    // arg!(...)
    ExpectedArgumentGotOption(&'static str, &'static str, String),  // "error: expected <{}:{}>, found '{}', which is an option\n{}"
    ExpectedArgumentGotEol(&'static str, &'static str),
    ArgumentWasNotParsable(&'static str, &'static str, String),  // "error: expected <{}:{}>, found {}, which could not be parsed as {}\n{}"

    // maybe!(...)
    OptionalArgumentWasNotParsable(&'static str, &'static str, String),

    // branch!(...)
    ExpectedBranchGotOption(&'static str, String, String),  // "error: expected <{}:{}>, found '{}', which is an option\n{}"
    ExpectedBranchGotEol(&'static str, String),
    BranchWasNotValid(&'static str, String, String),

    // opt!(...)
    OptionValueWasNotParsable(String, String, Option<String>)
}
