use crate::my_cli::foo::say_hello;
use azure_core::auth::TokenCredential; 
use azure_identity::{DefaultAzureCredential};
use url::Url;
// use my_cli::hello;
mod my_cli;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    my_cli::hello();
    say_hello();
    my_cli::foo::say_hello();
    Ok(())
}