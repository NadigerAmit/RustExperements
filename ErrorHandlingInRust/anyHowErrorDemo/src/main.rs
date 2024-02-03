use anyhow::{Error, Result};
use anyhow::Context;

fn main() {
    if let Err(err) = run_application() {
        eprintln!("Error: {:?}", err);

        // Accessing the underlying cause of the error (if available)
        if let Some(cause) = err.source() {
            eprintln!("Underlying cause: {:?}", cause);
        }
        // Accessing the backtrace (if available)
        eprintln!("Backtrace: {:?}", err.backtrace());

    }
}

fn run_application() -> Result<()> {
    let result = do_something()?;
    println!("Result: {}", result);
    Ok(())
}

fn do_something() -> Result<i32> {
    let data = read_data().context("Failed to read data")?;
    if data > 10 {
        anyhow::bail!("Data is too large");
    }
    Ok(data)
}

fn read_data() -> Result<i32> {
    // Simulating a read operation that might fail
    Ok(101)
}

// Note execute below in the terminal for seeing the backtrace
//$export RUST_LIB_BACKTRACE=1
