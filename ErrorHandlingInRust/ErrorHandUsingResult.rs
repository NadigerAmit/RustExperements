// main.rs
fn main() {
    if let Err(err) = run_application() {
        eprintln!("Error: {:?}", err);
    }
}

fn run_application() -> Result<(), CustomError> {
    // Simulating an error in your application logic
    let result = do_something()?;
    println!("Result: {}", result);
    Ok(())
}

fn do_something() -> Result<i32, CustomError> {
    let data = read_data()?;
    // Some application logic that may result in an error
    if data > 10 {
        return Err(CustomError::DataTooLarge);
    }
    Ok(data)
}

fn read_data() -> Result<i32, CustomError> {
    // Simulating a read operation that might fail
    Ok(42)
}

// Define a custom error type
#[derive(Debug)]
enum CustomError {
    ReadError,
    DataTooLarge,
}
