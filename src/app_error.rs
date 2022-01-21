use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    reason: AppErrorReason,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        AppError {
            reason: AppErrorReason::new(message),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[AppError occurs]: {}", self.reason)
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.reason)
    }
}

#[derive(Debug)]
struct AppErrorReason {
    message: String,
}

impl AppErrorReason {
    fn new(message: &str) -> Self {
        AppErrorReason {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for AppErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppErrorReason {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_error() {
       println!("{}", AppError::new("Bad message"));
    }
}
