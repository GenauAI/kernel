use rusqlite::{Connection, Error, params};
use serde::{Deserialize, Serialize};
use dirs::data_dir;
use std::error::Error as StdError;
use std::path::PathBuf;
use dotenv::dotenv;
use async_openai::{
    types::CreateCompletionRequestArgs,
    Client,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub steps: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub workspace_id: i32,
    pub sender: String,
    pub text: String,
    pub created_at: String,
}

const WORKSPACE_ID: i32 = 1;

pub fn get_database_path() -> Result<PathBuf, Box<dyn StdError>> {
    let mut path = data_dir().ok_or("Could not find data directory.")?;
    path.push("genau");
    path.push("genau.db");
    Ok(path)
}

pub fn get_db() -> Result<Connection, Error> {
    let path = get_database_path().unwrap();
    let conn = Connection::open(path)?;
    create_tables(&conn)?;
    Ok(conn)
}

pub fn save_message(conn: &Connection, message: &Message) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO messages (workspace_id, sender, text, created_at) VALUES (?1, ?2, ?3, datetime('now'))",
        params![message.workspace_id, message.sender, message.text],
    )?;

    Ok(())
}

pub fn reset_database(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "DELETE FROM messages WHERE workspace_id = ?1",
        [WORKSPACE_ID],
    )?;
    conn.execute("DELETE FROM plans WHERE workspace_id = ?1", [WORKSPACE_ID])?;
    Ok(())
}

pub fn save_plan(conn: &Connection, workspace_id: i32, plan: &Plan) -> Result<(), Error> {
    let steps = serde_json::to_string(&plan.steps).unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO plans (workspace_id, steps) VALUES (?1, ?2)",
        params![workspace_id, steps],
    )?;

    Ok(())
}

pub fn get_messages(conn: &Connection) -> Result<Vec<Message>, Error> {
    let mut stmt =
        conn.prepare("SELECT * FROM messages WHERE workspace_id = ?1 ORDER BY id DESC")?;
    let rows = stmt.query_map([WORKSPACE_ID], |row| {
        Ok(Message {
            id: row.get(0)?,
            workspace_id: row.get(1)?,
            sender: row.get(2)?,
            text: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;

    rows.collect()
}

pub fn create_tables(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id              INTEGER PRIMARY KEY,
            workspace_id    INTEGER NOT NULL,
            sender          TEXT NOT NULL,
            text            TEXT NOT NULL,
            created_at      TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS plans (
            workspace_id    INTEGER PRIMARY KEY,
            steps           TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn get_plan(conn: &Connection) -> Result<Plan, Error> {
    let mut stmt = conn.prepare("SELECT steps FROM plans WHERE workspace_id = ?1")?;
    let mut rows = stmt.query_map([WORKSPACE_ID], |row| {
        let steps: String = row.get(0)?;
        let steps = serde_json::from_str(&steps).unwrap();
        Ok(Plan { steps })
    })?;

    if let Some(plan) = rows.next().transpose()? {
        Ok(plan)
    } else {
        let plan = Plan { steps: vec![] };
        save_plan(conn, WORKSPACE_ID, &plan)?;
        Ok(plan)
    }
}

pub async fn do_stuff() {
    println!("Hello, world!");
    dotenv().ok();

    let client = Client::new();

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt("Tell me the recipe of alfredo pasta")
        .max_tokens(40_u16)
        .build()
        .unwrap();

    // Call API
    let response = client
        .completions() // Get the API "group" (completions, images, etc.) from the client
        .create(request) // Make the API call in that "group"
        .await
        .unwrap();

    dbg!(response);
}
