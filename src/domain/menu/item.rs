#[derive(Clone)]
pub struct Command {
    pub icon: String,
    pub name: String,
    pub command: String,
}

#[derive(Clone)]
pub enum Item {
    Cmd(Command),
    Separator,
}

impl From<Command> for Item {
    fn from(c: Command) -> Self {
        Self::Cmd(c)
    }
}
