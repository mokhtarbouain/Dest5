

mod error {
    use std::fmt;

    #[derive(Debug)]
    pub enum Error {
        InvalidInput(String),
        MatrixError(String),
    }

    impl Error {
        pub fn new(error_type: Option<String>, error_message: String) -> Error {
            match error_type {
                Some(error) if error.to_lowercase() == "invalidinput" => Error::InvalidInput(error_message),
                Some(error) if error.to_lowercase() == "matrixerror" => Error::MatrixError(error_message),
                _ => Error::InvalidInput("Invalid error type".to_string()),
            }
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::InvalidInput(error) => write!(f, "Invalid input: {}", error),
                Error::MatrixError(error) => write!(f, "Matrix error: {}", error),
            }
        }
    }

    impl std::error::Error for Error {}
}