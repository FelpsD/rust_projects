use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Subject {
    id: i32,
    name: String,
}

fn main() -> Result<()> {
    let mut conn = Connection::open("subject.db")?;

    //create_table(&mut conn);

    let mut me = Subject {
        id: 0,
        name: "Steven".to_string(),
    };
    insert_subject(&mut me, &mut conn);

    let mut stmt = conn.prepare("SELECT id, name, data FROM subject")?;
    let subject_iter = stmt.query_map([], |row| {
        Ok(Subject {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for subject in subject_iter {
        println!("Found subject {:?}", subject.unwrap());
    }
    Ok(())
}

fn create_table(conn: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE subject (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    Ok(())
}

fn insert_subject(me: &mut Subject, conn: &mut Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT INTO subject (name) VALUES (?1)",
        (me.name.clone(),),
    )?;
    Ok(())
}