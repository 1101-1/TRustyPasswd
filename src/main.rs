use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <add/delete/show> <url/username> <passwd>");
        return Ok(());
    }

    match args[1].trim() {
        "add" => {
            match args.get(2) {
                Some(name) => {
                    match args.get(3) {
                        Some(passwd) => return create_note(name.trim(), passwd.trim()),
                        None => {
                            println!("Usage:");
                            println!("cargo run -- <add/delete/show> <url/username> <passwd>");
                            return Err("Third arg is not found".into());
                        }
                    }
                }
                None => {
                    println!("Usage:");
                    println!("cargo run -- <add/delete/show> <url/username> <passwd>");
                    return Err("Second arg is not found".into());
                }
            }
            
        },
        "delete" => {
            match args.get(2) {
                Some(name) => return delete_note(name.trim()),
                None => {
                    println!("Usage:");
                    println!("cargo run -- <add/delete/show> <url/username> <passwd>");
                    return Err("Empty name to delete".into());
                }
            }
        },
        "show" => {
            match args.get(2) {
                Some(name) => return show(name.trim()),
                None => {
                    println!("Usage:");
                    println!("cargo run -- <add/delete/show> <url/username> <passwd>");
                    return Err("Empty name to show".into());
                }
            }
        },
        _ => {
            println!("Usage:");
            println!("cargo run -- <add/delete/show> <url/username> <passwd>");
            return Err("Incorrect args to run the programm".into());
        }
    }
}   

fn create_note(name: &str, passwd: &str) -> Result<(), Box<dyn Error>> {

    Err("args not found".into())
}

fn delete_note(name: &str) -> Result<(), Box<dyn Error>> {

    Ok(())
}

fn show(name: &str) -> Result<(), Box<dyn Error>> {


    Ok(())
}