use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct LockError;

impl Display for LockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LockError - could not acquire Mutex lock")
    }
}
impl Error for LockError {}