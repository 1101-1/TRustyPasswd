use std::error::Error;

use rusqlite::{named_params, Connection};

#[derive(Debug)]
struct Notes {
    name: String,
    passwd: String,
    url: String,
}

fn connection_db() -> Result<Connection, rusqlite::Error> {
    Connection::open("./db/data.db")
}

pub fn create_note(
    name: String,
    passwd: String,
    url: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let conn = connection_db()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            passwd  TEXT NOT NULL,
            url  TEXT
        )",
        (),
    )?;

    let url = if let Some(url) = url {
        url
    } else {
        "None".to_string()
    };

    let data: Notes = Notes {
        name: name.clone(),
        passwd: passwd.clone(),
        url: url.clone(),
    };

    conn.execute(
        "INSERT INTO notes (name, passwd, url) VALUES (?1, ?2, ?3)",
        (&data.name, &data.passwd, &data.url),
    )?;

    println!(
        "Inserted: username: {}, password: {}, url: {}",
        name, passwd, url
    );

    Ok(())
}

pub fn delete_note(name_or_id: String) -> Result<(), Box<dyn Error>> {
    let conn = connection_db()?;

    match name_or_id.parse::<u32>() {
        Ok(id) => {
           let mut stmt = conn.prepare("DELETE FROM notes WHERE id = ?")?;
            match stmt.execute([id]) {
                Ok(_) => {
                    println!("Note deleted");
                    return Ok(());
                },
                Err(_err) => return Err("Id does not exist".into())
            }; 
        }
        Err(_err) => {
            let mut stmt = conn.prepare("DELETE FROM notes WHERE name = ?")?;
            match stmt.execute([name_or_id]) {
                Ok(_) => {
                    println!("Notes deleted");
                    return Ok(());
                },
                Err(_err) => return Err("Name does not exist".into())
            };
        }
    }

}

pub fn show_note(name: String) -> Result<(), Box<dyn Error>> {
    let conn = connection_db()?;

    let mut stmt = conn.prepare("SELECT * FROM notes WHERE name = :name")?;
    let mut rows = stmt.query(named_params! {":name": name})?;

    let mut found_name = false;
    while let Some(row) = rows.next()? {
        found_name = true;
        println!(
            "Found result: id: {}, username: {}, password: {}, url: {}",
            row.get::<usize, u32>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
            row.get::<usize, String>(3)?
        )
    }

    if !found_name {
        return Err("Username does not exist. You can add it by command: cargo run -- add <username> <passwd> <url>".into());
    }

    Ok(())
}
