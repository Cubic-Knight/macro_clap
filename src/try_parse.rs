pub trait TryParse where Self: Sized {
    fn try_parse(s: String) -> Result<Self, ()>;
}


impl TryParse for bool {
    fn try_parse(s: String) -> Result<Self, ()> {
        match s.as_str() {
            "true" | "yes" | "Y" => Ok(true),
            "false" | "no" | "N" => Ok(false),
            _ => Err(())
        }
    }
}

macro_rules! impl_tryparse_for_types {
    ( $( $type:tt ),* ) => {
        $(
            impl TryParse for $type {
                fn try_parse(s: String) -> Result<Self, ()> {
                    s.parse::<$type>().map_err(|_| ())
                }
            }
        )*
    };
}

impl_tryparse_for_types!(
    String,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
);


impl<T> TryParse for Option<T>
where T: TryParse {
    fn try_parse(s: String) -> Result<Self, ()> {
        T::try_parse(s).map(Some)
    }
}
