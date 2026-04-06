// Imports
use clap::{Parser, Subcommand};

// Modules
mod client;
mod services {
    pub mod apps;
    pub mod auth;
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
    GenerateDeviceCode {
        #[arg(long)]
        client_id: String,
    },
    GenerateTokens {
        #[arg(long)]
        username: Option<String>,

        #[arg(long)]
        password: Option<String>,

        #[arg(long)]
        client_id: Option<String>,

        #[arg(long)]
        client_key: Option<String>,

        #[arg(long)]
        grant_type: Option<String>,

        #[arg(long)]
        redirect_uri: Option<String>,

        #[arg(long)]
        code: Option<String>,

        #[arg(long)]
        device_code: Option<String>,

        #[arg(long)]
        refresh_token: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse(); // Clap reads the command line arguments, validates them, then contructs a Cli
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

            AuthCommands::GenerateDeviceCode { client_id } => {
                let request_body = services::auth::NewDeviceCode {
                    client_id,
                };

                let body = services::auth::generate_device_code(&client, &request_body)?;
                println!("status: {}", body.status);
                println!("message: {}", body.message);
                println!("client_id: {}", body.result.client_id);
                println!("device_code: {}", body.result.device_code);
                println!("user_code: {}", body.result.user_code);
                println!("expires_in: {}", body.result.expires_in);
                println!("verification_uri: {}", body.result.verification_uri);
            }

            AuthCommands::GenerateTokens {
                username,
                password,
                client_id,
                client_key,
                grant_type,
                redirect_uri,
                code,
                device_code,
                refresh_token,
            } => {
                let request_body = services::auth::NewToken {
                    username,
                    password,
                    client_id,
                    client_key,
                    grant_type,
                    redirect_uri,
                    code,
                    device_code,
                    refresh_token,
                };

                let body = services::auth::generate_tokens(&client, &request_body)?;
                println!("status: {}", body.status);
                println!("message: {}", body.message);
                println!("access_token: {}", body.result.access_token.access_token);
                println!("id_token: {}", body.result.access_token.id_token);
                println!("expires_at: {}", body.result.access_token.expires_at);
                println!("expires_in: {}", body.result.access_token.expires_in);
                println!("jti: {}", body.result.access_token.jti);
            }
        },
    }

    Ok(())
}