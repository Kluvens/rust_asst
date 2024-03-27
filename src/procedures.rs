use crate::commands::Command;

#[derive(Debug)]
pub struct DummyProcedure {
    pub args: Vec<String>,
    pub commands: Vec<Command>,
}
