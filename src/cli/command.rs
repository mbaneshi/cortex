#[derive(Debug)]
pub struct Command {
    pub id: i32,
    pub command: String,
}

impl Command {
    pub fn new(id: i32, command: String) -> Self {
        Command { id, command }
    }
}
