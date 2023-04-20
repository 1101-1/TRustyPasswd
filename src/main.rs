use std::{env, error::Error};

mod connection_to_db;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run -- <add/delete/show> <username> <passwd> <url>");
        return Ok(());
    }

    match args[1].trim() {
        "add" => match args.get(2) {
            Some(name) => match args.get(3) {
                Some(passwd) => {
                    if let Some(url) = args.get(4) {
                        return connection_to_db::create_note(
                            name.to_string(),
                            passwd.to_string(),
                            Some(url.to_string()),
                        );
                    }
                    return connection_to_db::create_note(
                        name.to_string(),
                        passwd.to_string(),
                        None,
                    );
                }
                None => {
                    println!("Usage: cargo run -- <add> <username> <passwd> <url>");
                    return Err("Third arg is not found".into());
                }
            },
            None => {
                println!("Usage: cargo run -- <add> <username> <passwd> <url>");
                return Err("Second arg is not found".into());
            }
        },
        "delete" => match args.get(2) {
            Some(id) => return connection_to_db::delete_note(id.to_string()),
            None => {
                println!("Usage: cargo run -- delete <id/username>");
                return Err("Empty name to delete".into());
            }
        },
        "show" => match args.get(2) {
            Some(name) => return connection_to_db::show_note(name.to_string()),
            None => {
                println!("Usage: cargo run -- <show> <username/url>");
                return Err("Empty name to show".into());
            }
        },
        _ => {
            println!("Usage: cargo run -- <add/delete/show> <username> <passwd> <url>");
            return Err("Incorrect args to run the programm".into());
        }
    }
}
