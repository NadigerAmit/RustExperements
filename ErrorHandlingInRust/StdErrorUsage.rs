use std::error::Error;
use std::fmt;

// Define a custom error type
#[derive(Debug)]
struct CustomError {
    message: String,
}

// Implement the Display trait for custom error formatting
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom Error: {}", self.message)
    }
}

// Implement the Error trait
impl Error for CustomError {}

// Function that may return the custom error type
fn do_something() -> Result<(), CustomError> {
    // Simulating an error condition
    Err(CustomError {
        message: "Something went wrong.".to_string(),
    })
}

fn main() {
    // Attempt to perform an operation that may return the custom error
    if let Err(err) = do_something() {
        // Print the error using the Debug trait
        println!("Error: {:?}", err);

        // Print the error using the Display trait
        println!("Error: {}", err);

        // Access the error message directly
        println!("Error Message: {}", err.message);
    }
}
/*
amit@DESKTOP-TF687VE:~/OmRustPractice/AnyHowErrorExp/ErrorHandlingInRust$ ./StdErrorUsage
Error: CustomError { message: "Something went wrong." }
Error: Custom Error: Something went wrong.
Error Message: Something went wrong.
*/