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
    service: Services,
}

// Defines the services the user can use after the global options
#[derive(Subcommand, Debug)]
enum Services {
    Apps {
        #[command(subcommand)]
        command: AppsCommands,
    },
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
}

// Defines the operationId subcommands for the apps service
#[derive(Subcommand, Debug)]
enum AppsCommands {
    Healthcheck,
}

// Defines the operationId subcommands for the auth service
#[derive(Subcommand, Debug)]
enum AuthCommands {
    Hello,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse(); // Clap reads the command line arguments, validates the, then contructs a Cli
    let client = client::Client::new(cli.base_url);

    // Matches the service and operationId the user used and calls the appropriate function
    match cli.service {
        Services::Apps { command } => match command {
            AppsCommands::Healthcheck => {
                let body = services::apps::healthcheck(&client)?;
                println!("status: {}", body.status);
                println!("message: {}", body.message);
                println!("result: {}", body.result);
            }
        },

        Services::Auth { command } => match command {
            AuthCommands::Hello => {
                let body = services::auth::hello(&client)?;
                println!("status: {}", body.status);
                println!("message: {}", body.message);
                println!("version: {}", body.version);
            }
        },
    }

    Ok(())
}