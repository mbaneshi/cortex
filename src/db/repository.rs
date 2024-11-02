use crate::models::Command;
use rusqlite::{params, Connection, Result};

pub struct CommandRepository {
    conn: Connection,
}

impl CommandRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(CommandRepository { conn })
    }

    pub fn save_command(&self, command: &str) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO commands (command) VALUES (?1)",
            params![command],
        )
    }

    pub fn get_commands(&self) -> Result<Vec<Command>> {
        let mut stmt = self.conn.prepare("SELECT id, command FROM commands")?;
        let command_iter = stmt.query_map([], |row| {
            Ok(Command {
                id: row.get(0)?,
                command: row.get(1)?,
            })
        })?;

        command_iter.collect()
    }
}
