use clap::{Parser, Subcommand};
mod vault;
use vault::Vault;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new password
    Add {
        service: String,
        password: String,
    },

    /// Get a stored password
    Get {
        service: String,
    },

    /// List all stored services
    List,
}


fn main() {
    let args = Cli::parse();
    let mut _vault = Vault::load("vault.json");

    match args.command {

        Commands::Add { service, password } => {
            println!("Adding password for service: {}, password: {}", service, password);
            _vault.add_account(service, password);
            _vault.save("vault.json");
        }

        Commands::Get { service } => {
            let account =  _vault.get_account(&service);
            match account {
                Some(acc) => {
                    println!("Service: {}, Password: {}", acc.service, acc.password);
                }
                None => {
                    println!("No account found for service: {}", service);
                }
            }

        }
        Commands::List => {
            println!("Listing all stored services");
            for acc in _vault.list_accounts() {
                println!("Service: {}, Password: {}", acc.service, acc.password);
            }
        }
    }
}
