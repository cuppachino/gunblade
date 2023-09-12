use crate::errors::{CredentialError, LcuError};

#[derive(Debug, Default)]
/// Credentials extracted from a LeagueClientUx process on the local machine.
pub struct Credentials {
    pub id: usize,
    pub port: u16,
    pub pass: String,
}

impl TryFrom<Vec<&String>> for Credentials {
    type Error = LcuError;

    fn try_from(cmd_arg: Vec<&String>) -> Result<Self, Self::Error> {
        if cmd_arg.len() != 3 {
            Err(LcuError::CredentialError(CredentialError::NotEnoughTokens))
        } else {
            let mut id = 0;
            let mut port = 0;
            let mut pass = String::new();
            for arg in cmd_arg {
                if let Some((key, val)) = arg.rsplit_once("=") {
                    match key {
                        "--app-pid" => id = val.parse::<usize>().unwrap(),
                        "--app-port" => port = val.parse::<u16>().unwrap(),
                        "--remoting-auth-token" => pass = val.to_string(),
                        _ => {}
                    }
                }
            }
            Ok(Credentials { id, port, pass })
        }
    }
}
