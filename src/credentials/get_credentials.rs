use crate::{
    errors::{CredentialError, LcuError},
    Credentials,
};
pub use sysinfo::{
    NetworkExt, NetworksExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt,
};

/// Scan the process list for "LeagueClientUx" processes and get a vector of `Credentials`
/// that can be used to connect to the LCU.
pub fn get_credentials() -> Result<Vec<Result<Credentials, LcuError>>, LcuError> {
    const APP_PID: &str = "--app-pid=";
    const APP_PORT: &str = "--app-port=";
    const REMOTING_AUTH_TOKEN: &str = "--remoting-auth-token=";
    const UX_PROCESS: &'static str = "LeagueClientUx";
    let mut sys = System::new_all();
    sys.refresh_processes();
    let auth_tokens = sys
        .processes_by_name(UX_PROCESS)
        .map(|p| {
            let mut args = Vec::new();
            for arg in p.cmd() {
                if arg.starts_with(APP_PID) {
                    args.push(arg);
                } else if arg.starts_with(APP_PORT) {
                    args.push(arg);
                } else if arg.starts_with(REMOTING_AUTH_TOKEN) {
                    args.push(arg);
                }
            }
            Credentials::try_from(args)
        })
        .collect::<Vec<_>>();
    if auth_tokens.is_empty() {
        Err(LcuError::CredentialError(CredentialError::NotFound))
    } else {
        Ok(auth_tokens)
    }
}
