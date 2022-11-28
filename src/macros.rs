/// The main command-line argument parsing macro
/// 
/// Usage:
/// ```rust
/// cli!(
///     const ARG_PARSER: ArgParser<INTRODUCTION> = [
///         /* list of arguments */
///     ]
/// );
/// ```
#[macro_export]
macro_rules! cli {
    (
        const $const_name:tt: $const_type:tt < $introduction:tt > = [
            $(
                $macro_name:tt $macro_bang:tt (
                    $arg_var:tt as $res_type:tt $( $macro_params:tt )?
                ) 
            ),*
        ]
    ) => {
        struct $const_type;

        $(
            impl_type!($macro_name, $res_type $( $macro_params )?);
        )*

        impl $const_type {
            fn usage(self, program_name: String) -> String {
                #[allow(unused_mut)]
                let mut usage_strings = vec![ vec![" \x1b[33m".to_string(), program_name + "\x1b[m" ] ];
                $(
                    usage!(usage_strings: $macro_name, $arg_var, $res_type $( $macro_params )?);
                )*
                let mut full_strings = vec![ "USAGE: ".to_string() ];
                for string in usage_strings {
                    full_strings.push(string.join(" "))
                };
                full_strings.join("\n")
            }

            #[allow(unused_parens)]
            fn raw_parse_args(self) -> Result<( $( $res_type ),* ), (ArgParsingError, String)> {
                enum Argument {
                    Plain(String),
                    Option(String, Option<String>)
                }
                let mut args = std::env::args().collect::<Vec<_>>();
                let mut parsed_args = Vec::with_capacity(args.len());
                let mut temp = None;
                loop {
                    match args.pop() {
                        Some(opt_name) if opt_name.starts_with('-') => {
                            parsed_args.push( Argument::Option(opt_name, temp) );
                            temp = None;
                        },
                        Some(arg) => {
                            if let Some(arg2) = temp {
                                parsed_args.push( Argument::Plain(arg2) );
                            };
                            temp = Some(arg);
                        }
                        None => {
                            if let Some(arg2) = temp {
                                parsed_args.push( Argument::Plain(arg2) );
                            };
                            break;
                        }
                    };
                };
                let Some(Argument::Plain(program_name)) = parsed_args.pop() else {
                    return Err((ArgParsingError::NoProgramName, "".to_string()));
                };
                match parsed_args.pop() {
                    Some(Argument::Option(opt, _)) if opt.as_str() == "--help" => {
                        return Err((ArgParsingError::Help, self.usage(program_name)))
                    },
                    Some(arg) => parsed_args.push(arg),
                    None => {
                        return Err((ArgParsingError::Introduction($introduction), self.usage(program_name)))
                    }
                };
                $(
                    $macro_name $macro_bang (
                        $arg_var as $res_type from parsed_args with (self.usage(program_name)) $( $macro_params )?
                    );
                )*
                match parsed_args.pop() {
                    Some(Argument::Plain(arg)) => Err(
                        (ArgParsingError::UnknownArgument(arg), self.usage(program_name))
                    ),
                    Some(Argument::Option(opt, value)) => Err(
                        (ArgParsingError::UnknownOption(opt, value), self.usage(program_name))
                    ),
                    None => Ok(( $( $arg_var ),* ))
                }
            }

            #[allow(unused_parens)]
            fn parse_args(self) -> Result<( $( $res_type ),* ), String> {
                let (error, usage) = match self.raw_parse_args() {
                    Ok(res) => return Ok(res),
                    Err((error, usage)) => (error, usage)
                };
                let message = match error {
                    ArgParsingError::NoProgramName => format!(
                        "error: \x1b[31m\
                        no program name was given (how is that even possible?)\x1b[m"
                    ),
                    ArgParsingError::Introduction(intro) => intro.to_string(),
                    ArgParsingError::Help => "".to_string(),
                    ArgParsingError::UnknownArgument(arg) => format!(
                        "error: \x1b[31m\
                        unexpected argument '{}'\x1b[m",
                        arg
                    ),
                    ArgParsingError::UnknownOption(opt, None) => format!(
                        "error: \x1b[31m\
                        unexpected argument '{}'\x1b[m",
                        opt
                    ),
                    ArgParsingError::UnknownOption(opt, Some(value)) => format!(
                        "error: \x1b[31m\
                        unexpected argument '{} {}'\x1b[m",
                        opt, value
                    ),
                    ArgParsingError::ExpectedArgumentGotOption(name, typ, opt) => format!(
                        "error: \x1b[31m\
                        expected <{}:{}>, found '{}', which is an option\x1b[m",
                        name, typ, opt
                    ),
                    ArgParsingError::ExpectedArgumentGotEol(name, typ) => format!(
                        "error: \x1b[31m\
                        expected <{}:{}>, but ran out of arguments\x1b[m",
                        name, typ
                    ),
                    ArgParsingError::ArgumentWasNotParsable(name, typ, arg) => format!(
                        "error: \x1b[31m\
                        expected <{}:{}>, found {}, which could not be parsed as {}\x1b[m",
                        name, typ, arg, typ
                    ),
                    ArgParsingError::OptionalArgumentWasNotParsable(name, typ, arg) => format!(
                        "error: \x1b[31m\
                        expected optional [{}:{}], found {}, which could not be parsed as {}\
                        \x1b[m",
                        name, typ, arg, typ
                    ),
                    ArgParsingError::ExpectedBranchGotOption(name, typ, opt) => format!(
                        "error: \x1b[31m\
                        expected <{}:{}>, found '{}', which is an option\x1b[m",
                        name, typ, opt
                    ),
                    ArgParsingError::ExpectedBranchGotEol(name, typ) => format!(
                        "error: \x1b[31m\
                        expected <{}:{}>, but ran out of arguments\x1b[m",
                        name, typ
                    ),
                    ArgParsingError::BranchWasNotValid(name, typ, arg) => format!(
                        "error: \x1b[31m\
                        expected <{}:{}>, found {}, which is not a valid branch\x1b[m",
                        name, typ, arg
                    ),
                    ArgParsingError::OptionValueWasNotParsable(mess, opt, None) => format!(
                        "error: \x1b[31m\
                        {} while trying to parse '{}'\x1b[m",
                        mess, opt
                    ),
                    ArgParsingError::OptionValueWasNotParsable(mess, opt, Some(value)) => format!(
                        "error: \x1b[31m\
                        {} while trying to parse '{} {}'\x1b[m",
                        mess, opt, value
                    )
                };
                Err(message + "\n" + usage.as_str())
            }
        }

        const $const_name: $const_type = $const_type;
    };
}

/// Underlying macro to implement types. Not for use in code
#[macro_export]
macro_rules! impl_type {
    (arg, $type:tt) => {};
    (maybe, $type:tt) => {};
    (collect, $type:tt) => {};
    (
        branch, $type:tt {
            $( 
                $variant_kw:tt |> $variant:tt => {
                    $(
                        $macro_name:tt $macro_bang:tt (
                            $arg_var:tt as $res_type:tt $( $macro_params:tt )?
                        ) 
                    ),*
                }
            ),+
        }
    ) => {
        #[allow(unused_parens)]
        #[derive(Debug)]
        enum $type {
            $(
                $variant( $( $res_type ),* )
            ),+
        }

        $(
            $(
                impl_type!($macro_name, $res_type $( $macro_params )?);
            )*
        )+
    };
    (
        opt, $type:tt {
            $(
                $field:tt : $opt_kw:tt -> $opt_res_type:tt
            ),+
        }
    ) => {
        #[allow(unused_parens)]
        #[derive(Debug)]
        struct $type {
            $(
                $field: $opt_res_type
            ),+
        }
    };
}

/// Underlying macro to make usage strings. Not for use in code
#[macro_export]
macro_rules! usage {
    ($strings:tt : arg, $var:tt, $type:tt) => {
        for string in &mut $strings {
            let mut var_type = stringify!($type).to_string();
            var_type.retain(|c| !c.is_whitespace() && c != '(' && c != ')');
            string.push( 
                format!(
                    "\x1b[90m<\x1b[m{}\x1b[90m:\x1b[32m{}\x1b[90m>\x1b[m",
                    stringify!($var), var_type
                )
            );
        };
    };
    ($strings:tt : maybe, $var:tt, $type:tt) => {
        for string in &mut $strings {
            let mut var_type = stringify!($type).to_string();
            var_type.retain(|c| !c.is_whitespace() && c != '(' && c != ')');
            string.push(
                format!(
                    "\x1b[90m[\x1b[m{}\x1b[90m:\x1b[32m{}\x1b[90m]\x1b[m",
                    stringify!($var), var_type
                ) 
            );
        };
    };
    (
        $strings:tt : branch, $var:tt, $type:tt {
            $( $word:tt |> $variant:tt => {} ),+
        }
    ) => {
        for string in &mut $strings {
            string.push(
                format!(
                    "\x1b[90m<\x1b[m{}\x1b[90m:\x1b[34m{}\x1b[90m>\x1b[m",
                    stringify!($var),
                    [ $( $word ),+ ].join("\x1b[m|\x1b[34m")
                )
            );
        };
    };
    (
        $strings:tt : branch, $var:tt, $type:tt {
            $(
                $word:tt |> $variant:tt => {
                    $(
                        $macro_name:tt $macro_bang:tt (
                            $arg_var:tt as $res_type:tt
                            $( $macro_params:tt )?
                        )
                    ),*
                }
            ),+
        }
    ) => {
        let temp_strings = $strings;
        let mut $strings = vec![];
        $(
            let mut branch = vec![ vec![ "\x1b[34m".to_string() + $word + "\x1b[m" ] ];
            $(
                usage!(branch: $macro_name, $arg_var, $res_type $( $macro_params )?);
            )*
            for usage_start in &temp_strings {
                for br in &mut branch {
                    let mut element = usage_start.clone();
                    element.append(br);
                    $strings.push(element);
                }
            }
        )+
    };
    ($strings:tt : opt, $var:tt, $type:tt $rest:tt) => {
        for string in &mut $strings {
            string.push( format!("\x1b[90m[\x1b[mOPTIONS\x1b[90m]\x1b[m") );  // TODO: Better display
        };
    };
    ($strings:tt : collect, $var:tt, $type:tt) => {
        for string in &mut $strings {
            string.push( format!("\x1b[90m(...)\x1b[m") );
        };
    };
}

/// The simple argument
/// 
/// Usage:
/// ```rust
/// arg!(name as Type)
/// ```
#[macro_export]
macro_rules! arg {
    ($arg_var:tt as $res_type:tt from $args:tt with $usage_string:tt) => {
        let arg = match $args.pop() {
            Some(Argument::Plain(arg)) => arg,
            Some(Argument::Option(opt, _)) => return Err((
                ArgParsingError::ExpectedArgumentGotOption(stringify!($arg_var), stringify!($res_type), opt),
                $usage_string
            )),
            None => return Err((
                ArgParsingError::ExpectedArgumentGotEol(stringify!($arg_var), stringify!($res_type)),
                $usage_string
            ))
        };
        let $arg_var = match <$res_type>::try_parse(arg.clone()) {
            Ok(thing) => thing,
            Err(_) => return Err((
                ArgParsingError::ArgumentWasNotParsable(stringify!($arg_var), stringify!($res_type), arg),
                $usage_string
            ))
        };
    };
}

/// The argument that may be missing
/// 
/// Usage:
/// ```rust
/// maybe!(name as (Option<Type>))
/// ```
#[macro_export]
macro_rules! maybe {
    ($arg_var:tt as $res_type:tt from $args:tt with $usage_string:tt) => {
        let maybe_arg = match $args.pop() {
            Some(Argument::Plain(arg)) => Some(arg),
            Some(Argument::Option(opt, value)) => {
                $args.push(Argument::Option(opt, value));
                None
            },
            None => None
        };
        let $arg_var = match maybe_arg {
            Some(arg) => match <$res_type>::try_parse(arg.clone()) {
                Ok(thing) => thing,
                Err(_) => return Err((
                    ArgParsingError::OptionalArgumentWasNotParsable(stringify!($arg_var), stringify!($res_type), arg),
                    $usage_string
                ))
            },
            None => None
        };
    };
}

/// The intersection that separates two branches of arguments
/// 
/// Usage:
/// ```rust
/// branch!(name as BranchEnum {
///     "keyword_1" |> Variant1 => {
///         /* list of args if keyword_1 */
///     },
///     "keyword_2" |> Variant2 => {
///         /* list of args if keyword_2 */
///     },
///     // You can make as many keywords as you want
/// })
/// ```
#[macro_export]
macro_rules! branch {
    (
        $arg_var:tt as $res_type:tt from $args:tt with $usage_string:tt {
            $(
                $word:tt |> $variant:tt => {
                    $(
                        $macro_name:tt $macro_bang:tt (
                            $inner_arg_var:tt as $inner_res_type:tt
                            $( $inner_macro_params:tt )?
                        ) 
                    ),*
                }
            ),+
        }
    ) => {
        let arg = match $args.pop() {
            Some(Argument::Plain(arg)) => arg,
            Some(Argument::Option(opt, _)) => return Err((
                ArgParsingError::ExpectedBranchGotOption(stringify!($arg_var), [ $( $word ),+ ].join("|"), opt),
                $usage_string
            )),
            None => return Err((
                ArgParsingError::ExpectedBranchGotEol(stringify!($arg_var), [ $( $word ),+ ].join("|")),
                $usage_string
            ))
        };
        let $arg_var = match arg.as_str() {
            $(
                $word => {
                    $(
                        $macro_name $macro_bang (
                            $inner_arg_var as $inner_res_type from $args with $usage_string
                            $( $inner_macro_params )?
                        );
                    )*
                    <$res_type>::$variant( $( $inner_arg_var ),* )
                }
            ),+,
            _ => return Err((
                ArgParsingError::BranchWasNotValid(stringify!($arg_var), [ $( $word ),+ ].join("|"), arg),
                $usage_string
            ))
        };
    };
}

/// The matcher of options
/// 
/// Usage:
/// ```rust
/// opt!(name as OptionStruct {
///     option_1_name: [ /* list of all the keywords related to option 1 */ ] -> OptionType1,
///     option_2_name: [ /* list of all the keywords related to option 2 */ ] -> OptionType2,
///     // You can make as many keywords as you want
/// })
/// ```
#[macro_export]
macro_rules! opt {
    (
        $arg_var:tt as $res_type:tt from $args:tt with $usage_string:tt {
            $(
                $field:tt : [ $( $opt_kw:tt ),+ ] -> $opt_res_type:tt
            ),*
        }
    ) => {
        let mut $arg_var = $res_type {
            $(
                $field: <$opt_res_type>::receptacle_default()
            ),*
        };
        loop {
            let (opt, value) = match $args.pop() {
                Some(Argument::Plain(arg)) => {
                    $args.push(Argument::Plain(arg));
                    break;
                },
                Some(Argument::Option(opt, value)) => (opt, value),
                None => break
            };
            match opt.as_str() {
                $(
                    $( $opt_kw )|+ => match $arg_var.$field.receive_value(value.clone()) {
                        Ok(()) => (),
                        Err(message) => return Err((
                            ArgParsingError::OptionValueWasNotParsable(message, opt, value),
                            $usage_string
                        ))
                    }
                ),*,
                _ => {
                    $args.push(Argument::Option(opt, value));
                    break;
                }
            };
        };
    };
}

/// The collecter of all leftovers
/// 
/// Usage:
/// ```rust
/// collect!(rest as (Vec<String>))
/// ```
#[macro_export]
macro_rules! collect {
    ($arg_var:tt as $res_type:tt from $args:tt with $usage_string:tt) => {
        let mut temp = Vec::new();
        for arg in $args {
            match arg {
                Argument::Plain(arg) => temp.push(arg),
                Argument::Option(opt, None) => temp.push(opt),
                Argument::Option(opt, Some(value)) => {
                    temp.push(opt);
                    temp.push(value);
                }
            }
        }
        let $arg_var = temp.into_iter().collect::<$res_type>();
        let mut $args = Vec::new();
    };
}
