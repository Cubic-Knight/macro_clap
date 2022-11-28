pub trait OptionReceptacle {
    fn receptacle_default() -> Self;
    fn receive_value(&mut self, value: Option<String>) -> Result<(), String>;
}
/// Whether an option is mentionned or not
/// 
/// `Flag` is set has its `state` set to `false` by default
/// 
/// If the option is present one or more times in the arguments, its `state` will be set to `true`
#[derive(Debug)]
pub struct Flag {
    pub state: bool
}

impl OptionReceptacle for Flag {
    fn receptacle_default() -> Self {
        Flag { state: false }
    }
    fn receive_value(&mut self, _value: Option<String>) -> Result<(), String> {
        self.state = true;
        Ok(())
    }
}

/// How much all of the values of the option add up to
/// 
/// `Counter` is set has its `count` set to `0` by default
/// 
/// If the option is present without value, the `count` will be incremented by `1`
/// 
/// If the option is present with a integer value, the `count` will be increased by that value
#[derive(Debug)]
pub struct Counter<T>{
    pub count: T
}

impl<T> OptionReceptacle for Counter<T>
where T: From<u8> + std::str::FromStr + std::ops::AddAssign<T> {
    fn receptacle_default() -> Self {
        Counter { count: T::from(0) }
    }
    fn receive_value(&mut self, value: Option<String>) -> Result<(), String> {
        match value {
            Some(string) => {
                self.count += string.parse::<T>()
                    .map_err(|_| "Could not parse")?
            },
            None => self.count += T::from(1)
        };
        Ok(())
    }
}

/// How many times an option is mentionned
/// 
/// `FlagCounter` is set has its `count` set to `0` by default
/// 
/// If the option is present without value, the `count` will be incremented by `1`
/// 
/// If the option is present with a value, unlike `Counter`, it will result in an error
#[derive(Debug)]
pub struct FlagCounter<T>{
    pub count: T
}

impl<T> OptionReceptacle for FlagCounter<T>
where T: From<u8> + std::str::FromStr + std::ops::AddAssign<T> {
    fn receptacle_default() -> Self {
        FlagCounter { count: T::from(0) }
    }
    fn receive_value(&mut self, _value: Option<String>) -> Result<(), String> {
        self.count += T::from(1);
        Ok(())
    }
}

/// The first value given to the option
/// 
/// `GrabFirst` will return the value of the first occurence of the option
/// 
/// If the option is not mentionned, its field `first` will be None
#[derive(Debug)]
pub struct GrabFirst<T> {
    pub first: Option<T>,
}

impl<T> OptionReceptacle for GrabFirst<T>
where T: std::str::FromStr {
    fn receptacle_default() -> Self {
        GrabFirst { first: None }
    }
    fn receive_value(&mut self, value: Option<String>) -> Result<(), String> {
        match value {
            Some(string) if self.first.is_none() => {
                let res = string.parse::<T>().map_err(|_| "Could not parse")?;
                self.first = Some(res);
            },
            Some(_) => (),
            None => return Err("Need option value".to_string())
        };
        Ok(())
    }
}

/// The last value given to the option
///
/// `GrabLast` will return the value of the last occurence of the option
/// 
/// If the option is not mentionned, its field `last` will be None
#[derive(Debug)]
pub struct GrabLast<T> {
    pub last: Option<T>,
}

impl<T> OptionReceptacle for GrabLast<T>
where T: std::str::FromStr {
    fn receptacle_default() -> Self {
        GrabLast { last: None }
    }
    fn receive_value(&mut self, value: Option<String>) -> Result<(), String> {
        match value {
            Some(string) => {
                let res = string.parse::<T>().map_err(|_| "Could not parse")?;
                self.last = Some(res);
            },
            None => return Err("Need option value".to_string())
        };
        Ok(())
    }
}

/// All values given to the option
///
/// `GrabAll` will return the value of all the occurences of the option, in a `Vec<_>`
/// 
/// If the option is not mentionned, its field `items` will be a `Vec` of length `0`
#[derive(Debug)]
pub struct GrabAll<T> {
    pub items: Vec<T>,
}

impl<T> OptionReceptacle for GrabAll<T>
where T: std::str::FromStr {
    fn receptacle_default() -> Self {
        GrabAll { items: vec![] }
    }
    fn receive_value(&mut self, value: Option<String>) -> Result<(), String> {
        match value {
            Some(string) => {
                let res = string.parse::<T>().map_err(|_| "Could not parse")?;
                self.items.push(res);
            },
            None => return Err("Need option value".to_string())
        };
        Ok(())
    }
}
