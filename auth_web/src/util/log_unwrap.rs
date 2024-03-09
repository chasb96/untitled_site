use std::fmt::Display;
use std::fmt::Debug;
use log::error;

pub trait LogUnwrap {
    type Ok;

    fn log_unwrap(self) -> Self::Ok;
}

impl<T, E> LogUnwrap for Result<T, E> 
where
    E: Debug + Display
{
    type Ok = T;

    fn log_unwrap(self) -> T {
        self.inspect_err(|e| error!("{}", e))
            .unwrap()
    }
}