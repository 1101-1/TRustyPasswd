use clap::{Parser, builder::Str};
use std::error::Error;

mod connection_to_db;
mod generate_passwd;

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
    #[arg(short, long)]
    passwd: Option<String>,
    #[arg(short, long)]
    url: Option<String>,
    #[arg(short, long, default_value = "false")]
    generate_passwd: String
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut env_args = Cli::parse();

    match env_args.arg.as_str() {
        "add" => {
            if env_args.passwd == None && env_args.generate_passwd == "true" {
                env_args.passwd = Some(generate_passwd::generate_passwd())
            }
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
            return Err("Empty args.".into());
        }
        _ => {
            println!(
                "Usage: ./trusty_passwd -a <add/delete/show> -n <username> -s <service> -p <passwd> -u <url> -e <false(by default it is true)>"
            );
            return Err("Incorrect args to run the programm".into());
        }
    }
}
