use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io;

struct AppError {
    code: usize,
    message: String,
}

impl AppError {
    fn new(code: usize, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_message = match self.code {
            404 => "resource not found",
            _ => self.message.as_str()
        };
        write!(f, "{}", error_message)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomError {{ code: {}, message: {} }}", self.code, self.message)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self {
            code: 500,
            message: error.to_string(),
        }
    }
}

fn produce_error() -> Result<(), AppError> {
    Err(AppError::new(404, "produced error"))
}

fn main() -> Result<(), AppError> {
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("no error"),
    }

    eprintln!("{:?}", produce_error());
    eprintln!("{:#?}", produce_error());

    let _file = File::open("nonexistent_file.txt")?;
    Ok(())
}
