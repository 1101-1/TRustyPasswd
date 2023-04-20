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

pub fn delete_note(name: String) -> Result<(), Box<dyn Error>> {
    
    Ok(())
}

pub fn show_note(name: String) -> Result<(), Box<dyn Error>> {
    let conn = connection_db()?;

    let mut stmt = conn.prepare("SELECT * FROM notes WHERE name = :name")?;
    let mut rows = stmt.query(named_params! { ":name": name})?;

    while let Some(row) = rows.next()? {
        println!(
            "Founded result by NAME {}: username: {}, password: {}, url {}",
            name,
            row.get::<usize, String>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?
        )
    }
    Ok(())
}
