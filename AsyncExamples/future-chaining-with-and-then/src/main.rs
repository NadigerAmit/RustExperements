use futures::future::FutureExt; // Import the extension traits
use futures::future::TryFutureExt; // Import the extension traits
use tokio::time::{sleep, Duration};


async fn async_operation() -> Result<i32, String> {
  sleep(Duration::from_secs(3)).await;
  Ok(101)
}

#[tokio::main]
async fn main() {
  let future1 = async_operation().await.and_then(|result| {
    if result > 99 {
      println!("Value is greater than 99");
      Ok(("Value is greater than 99").to_string())
    } else {
      println!("Value is less than 99");
      Err(("Value is less than 99").to_string())
    }
  });
} 