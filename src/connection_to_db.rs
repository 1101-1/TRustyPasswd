use std::error::Error;

use rusqlite::{named_params, Connection};

#[derive(Debug)]
struct Notes {
    service: String,
    name: String,
    passwd: String,
    url: String,
}

fn connection_db() -> Result<Connection, rusqlite::Error> {
    if cfg!(target_os = "windows") {
        Connection::open(r".\db\data.db")
    } else {
        Connection::open("./db/data.db")
    }
}

pub fn create_note(
    name: Option<String>,
    service: Option<String>,
    passwd: Option<String>,
    url: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let conn = connection_db()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            service  TEXT,
            passwd  TEXT,
            url  TEXT
        )",
        (),
    )?;

    let name = if let Some(name) = name {
        name
    } else {
        "None".to_string()
    };

    let service = if let Some(service) = service {
        service
    } else {
        "None".to_string()
    };

    let passwd = if let Some(passwd) = passwd {
        passwd
    } else {
        "None".to_string()
    };

    let url = if let Some(url) = url {
        url
    } else {
        "None".to_string()
    };

    let data: Notes = Notes {
        name: name.clone(),
        service: service.clone(),
        passwd: passwd.clone(),
        url: url.clone(),
    };

    conn.execute(
        "INSERT INTO notes (name, service, passwd, url) VALUES (?1, ?2, ?3, ?4)",
        (&data.name, &data.service, &data.passwd, &data.url),
    )?;

    println!(
        "Inserted: service: {}, username: {}, password: {}, url: {}",
        service, name, passwd, url
    );

    Ok(())
}

pub fn delete_note(name_or_service_or_id: String, prop: &str) -> Result<(), Box<dyn Error>> {
    let conn = connection_db()?;

    match prop {
        "id" => {
            let id = &name_or_service_or_id.parse::<u32>().unwrap();
            let mut stmt = conn.prepare("DELETE FROM notes WHERE id = ?")?;
            match stmt.execute([id]) {
                Ok(_) => {
                    println!("Note deleted");
                    return Ok(());
                }
                Err(_err) => return Err("Id does not exist".into()),
            };
        }
        "service" => {
            let mut stmt = conn.prepare("DELETE FROM notes WHERE service = ?")?;
            match stmt.execute([name_or_service_or_id]) {
                Ok(_) => {
                    println!("Note deleted");
                    return Ok(());
                }
                Err(_err) => return Err("Id does not exist".into()),
            };
        }
        "name" => {
            let mut stmt = conn.prepare("DELETE FROM notes WHERE name = ?")?;
            match stmt.execute([name_or_service_or_id]) {
                Ok(_) => {
                    println!("Note deleted");
                    return Ok(());
                }
                Err(_err) => return Err("Id does not exist".into()),
            };
        }
        _ => return Err("Unexpected error".into()),
    }
}

pub fn show_note(name_or_service: String, prop: &str) -> Result<(), Box<dyn Error>> {
    let conn = connection_db()?;

    match prop {
        "name" => show_note_name(&conn, &name_or_service),
        "service" => show_note_service(&conn, &name_or_service),
        _ => return Err("Unexpected error".into()),
    }
}

fn show_note_service(conn: &Connection, service: &String) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT * FROM notes WHERE service = :service")?;
    let mut rows = stmt.query(named_params! {":service": service})?;

    let mut found_name = false;
    while let Some(row) = rows.next()? {
        found_name = true;
        println!(
            "Found result: id: {}, username: {}, service: {}, password: {}, url: {}",
            row.get::<usize, u32>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
            row.get::<usize, String>(3)?,
            row.get::<usize, String>(4)?
        )
    }

    if !found_name {
        return Err("Username does not exist. You can add it by command: ./trusty_passwd -a <add> <username> <service> <passwd> <url>".into());
    }

    Ok(())
}

fn show_note_name(conn: &Connection, name: &String) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT * FROM notes WHERE name = :name")?;
    let mut rows = stmt.query(named_params! {":name": name})?;

    let mut found_name = false;
    while let Some(row) = rows.next()? {
        found_name = true;
        println!(
            "Found result: id: {}, username: {}, service: {}, password: {}, url: {}",
            row.get::<usize, u32>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
            row.get::<usize, String>(3)?,
            row.get::<usize, String>(4)?
        )
    }

    if !found_name {
        return Err("Username does not exist. You can add it by command: ./trusty_passwd -a <add> <username> <service> <passwd> <url>".into());
    }

    Ok(())
}
