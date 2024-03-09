pub trait Invert<T, E> {
    type Result;

    fn invert(self) -> Self::Result;
}

impl<T, E> Invert<T, E> for Result<Option<T>, E> {
    type Result = Option<Result<T, E>>;

    fn invert(self) -> Self::Result {
        match self {
            Ok(Some(t)) => Some(Ok(t)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<T, E> Invert<T, E> for Option<Result<T, E>> {
    type Result = Result<Option<T>, E>;

    fn invert(self) -> Self::Result {
        match self {
            Some(Ok(t)) => Ok(Some(t)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }
}

