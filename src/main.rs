use clap::Parser;
use std::error::Error;

mod connection_to_db;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    arg: String,
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long)]
    service: Option<String>,
    #[arg(long)]
    id: Option<String>,
    passwd: Option<String>,
    url: Option<String>,
}
fn main() -> Result<(), Box<dyn Error>> {
    let env_args = Cli::parse();

    match env_args.arg.as_str() {
        "add" => {
            return connection_to_db::create_note(
                env_args.name,
                env_args.service,
                env_args.passwd,
                env_args.url,
            );
        }
        "show" => {
            if let Some(service) = env_args.service {
                return connection_to_db::show_note(service, "service");
            }
            if let Some(name) = env_args.name {
                return connection_to_db::show_note(name, "name");
            }
            return Err("Empty args.".into());
        }
        "delete" => {
            if let Some(service) = env_args.service {
                return connection_to_db::delete_note(service, "service");
            }
            if let Some(name) = env_args.name {
                return connection_to_db::delete_note(name, "name");
            }
            if let Some(id) = env_args.id {
                return connection_to_db::delete_note(id, "id");
            }
            return Err("Empty args. Use".into());
        }
        _ => {
            println!(
                "Usage: ./trusty_passwd -a <add/delete/show> <username> <service> <passwd> <url>"
            );
            return Err("Incorrect args to run the programm".into());
        }
    }
}
