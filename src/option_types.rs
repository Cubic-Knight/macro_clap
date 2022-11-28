pub trait OptionReceptacle {
    fn receptacle_default() -> Self;
    fn receive_value(&mut self, value: Option<String>) -> Result<(), String>;
}

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
