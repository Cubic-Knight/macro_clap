# macro_clap

A quick and easy way to make command-line argument parsing, with automatic colored usage string,
as well as support for options and argument branches

## Example

```rust
use macro_clap::*;

const INTRODUCTION: &'static str = "Whatever you want your intro to be";

cli!(
    const ARG_PARSER: ArgParser<INTRODUCTION> = [
        branch!(action as ActionEnum {
            "copy" |> Copy => {
                arg!(source as String),
                arg!(dest as String)
            },
            "delete" |> Delete => {
                arg!(source as String)
            },
            "modify" |> Modify => {
                branch!(modif_type as ModifType {
                    "replace" |> Replace => {},
                    "append" |> Append => {}
                }),
                arg!(source as String),
                arg!(text as String),
                maybe!(encoding as (Option<String>)),
                opt!(options as ModifyOptions {
                    interleave: ["--interleave"] -> (GrabLast<char>)
                })
            }
        }),
        opt!(options as OptionStruct {
            verbose: ["-v", "--verbose"] -> (Counter<u8>),
            explain: ["-x", "--explain"] -> Flag
        }),
        collect!(rest as (Vec<String>))
    ]
);

fn main() {
    match ARG_PARSER.parse_args() {
        Ok(args) => {
            println!("{:?}", args);
        },
        Err(e) => {
            println!("{e}");
            return;
        }
    };
}
```

## Usage
If the `Err(String)` case of `ARG_PARSER.parse_args()` is printed in the console, your code will have the following behavior:
- Calling `your_crate` in the command line will print out the introduction in the console, as well as the usage string
- Calling `your_crate --help` will print only the usage string
- Calling `your_crate (some incorrect argument configuration)` will print an error and the usage string
- If too many arguments are given, and no `collect!` macro has been setup, it will print an error and the usage string

## Limitations
In macro_clap, all arguments starting with a '-' are considered options
and all other are considered plain arguments.
You also have very little power over the error messages,
and they might sometimes not be very useful to an inexperienced user.
That is because macro_clap prioritizes shortness of code over usability, and
is made for people that want to quickly code a functional CLI without thinking too
much on the implementation details, to make a prototype version for example.

## Syntax
```rust
// cli! is the root of your command line interface
// SomeUniqueType is just a type that the macro needs to make the cli
// You CAN make multiple cli! in the same file or same crate, as long as their types are not shared
// YOUR_INTRODUCTION must be either a &'static str or a const &'static str
// Once your cli! macro is constructed, use YOUR_CONST_NAME.parse_args() to parse the arguments
// The return value of YOUR_CONST_NAME.parse_args() is Result<(Arguments, String)>
//   where Arguments is the tuple of the result of the parsing of all arguments
cli!(
    const YOUR_CONST_NAME: SomeUniqueType<YOUR_INTRODUCTION> = [
        /* list of all the arguments */
    ]
)

// This tells the macro to wait for an argument and to parse it as ArgType
// If no argument is passed, the macro will fail
// ArgType must be String, bool, or an integer
arg!(arg_name as ArgType)

// This tells the macro to wait for an argument and to parse it as Some(ArgType)
// However, unlike arg!, maybe! will not fail if no argument is given, but return None instead
// ArgType must be String, bool, or an integer
// Please do not forget to wrap ArgType in an Option and to surrond everything by parentheses
maybe!(maybe_arg_name as (Option<ArgType>))

// This tells the macro to wait for a keyword in the list
// If no keyword is passed, or if the keyword is not present in the list, the macro will fail
// BranchEnum is the type that will be returned by the branch! macro
// BranchEnum is automatically implemented by the macro, so make sure that it is unique
// Variants of BranchEnum are tuple variants, and contain the results of the arguments specific to their own path
branch!(branch_name as BranchEnum {
    "keyword_1" |> Variant1 => {
        /* list of args if keyword_1 */
    },
    "keyword_2" |> Variant2 => {
        /* list of args if keyword_2 */
    },
    // You can make as many keywords as you want
}),

// This tells the macro to start parsing as many options as possible
// When the macro stops finding options, it just continues
// Lists of keywords related to options can be for example ["-v", "--verbose"]
// OptionTypes dictates how the options are handled
// OptionTypes are Counter, Flag, FlagCounter, GrabFirst, GrabLast and GrabAll
// Please see their documentation for more information about how they operate
// Please make sure that every OptionType is surrounded by parenthesis if it is a generic type
//  i.e. (Counter<i8>) or (GrabFirst<String>)
opt!(option_group_name as OptionStruct {
    option_1_name: [ /* list of all the keywords related to option 1 */ ] -> OptionType1,
    option_2_name: [ /* list of all the keywords related to option 2 */ ] -> OptionType2,
    // You can make as many options as you want
}),

// This will dump every argument left over by the previous parsing into a Vec<String>
// Please note that the return type of collect! MUST be (Vec<String>)
// Please make sure that nothing follows a collect!, as it will never recieve any arguments
collect!(rest as (Vec<String>))
```
