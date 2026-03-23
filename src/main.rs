// Imports
use clap::{Parser, Subcommand};

// Modules
mod client;
mod device_code;
mod services {
    pub mod apps;
    pub mod auth;
}
mod utils {
    pub mod error;
}

// Defines the Cli struct
#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, default_value = "https://dev.tapis.io")] // Sets a base url to use if the user doesn't define one
    base_url: String,

    #[command(subcommand)] // tells clap to expect a subcommand after the global options
    command: Commands,
}

// Defines the subcommands the user can use after the global options
#[derive(Subcommand, Debug)]
enum Commands {
    Healthcheck,
    AuthHello,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse(); // Clap reads the command line arguments, validates the, then contructs a Cli
    let client = client::Client::new(cli.base_url);

    // Matches the subcommand the user used and calls the appropriate function
    match cli.command {
        Commands::Healthcheck => {
            let body = services::apps::healthcheck(&client)?;
            println!("status: {}", body.status);
            println!("message: {}", body.message);
            println!("result: {}", body.result);
        }

        Commands::AuthHello => {
            let body = services::auth::hello(&client)?;
            println!("status: {}", body.status);
            println!("message: {}", body.message);
            println!("version: {}", body.version);
        }
    }

    Ok(())
}