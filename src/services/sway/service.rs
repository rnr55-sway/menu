use super::ConnectionError;
use super::RunError;

pub struct Sway {
    connection: swayipc::Connection
}

impl Sway {
    pub fn new() -> Result<Self, ConnectionError> {
        Ok(Sway {
            connection: swayipc::Connection::new()?
        })
    }

    pub fn run(&mut self, cmd: &String) -> Result<&mut Self, RunError> {
        let res = self.connection.run_command(cmd)?;

        for r in res.iter() {
            if !r.success {
                let err = match &r.error {
                    Some(s) => RunError::Failed(s.clone()),
                    None => RunError::UnknownErr
                };

                return Err(err);
            }
        }

        Ok(self)
    }
}
