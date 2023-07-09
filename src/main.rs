use crate::my_cli::foo::say_hello;
use azure_core::auth::TokenCredential;
use clap::Parser;
mod azure;
mod open_ai;
use azure_identity::DefaultAzureCredential;
use url::Url;
// use my_cli::hello;
mod my_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    my_cli::hello();
    say_hello();
    my_cli::foo::say_hello();
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);
    let args2 = crate::my_cli::Cli::parse();
    println!("args2: {:?}", args2);
    // azure::do_stuff().await?;
    open_ai::example().await;
    Ok(())
}
