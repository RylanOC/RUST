
lazy_static! {
    pub static ref ADDRESS: String = address();
    pub static ref CERT_FILE: String = cert_file();
    pub static ref KEY_FILE: String = priv_key_file();
    pub static ref CLIENT_ID: String = cliet_id();
}

/// Default address to use if it is not already set by an environment variable.
const DEFAULT_ADDRESS: &'static str = "127.0.0.1:8443";
const ADDRESS_ENV_VAR: &'static str = "BIND_TO";

/// Sets the log level as an env variable if it is not currently set.
const DEFAULT_LOG_LEVEL: &'static str = "info";
const LOG_LEVEL_ENV_VAR: &'static str = "RUST_LOG"; // this cannot change.

/// Client ID given by Spotify
const DEFAULT_CLIENT_ID: &'static str = "1de388fded5c43b68f60fcec9a81c956";
const CLIENT_ID_ENV_VAR: &'static str = "CLIENT_ID"; // will break env.bash if changed.

/// Default cert file location. Can be overridden with "CERT_FILE" env variable.
const DEFAULT_CERT_FILE: &'static str = "cert.pem";
const CERT_FILE_ENV_VAR: &'static str = "CERT_FILE";

/// Default private key location. Can be overridden with "PRIV_KEY" env variable.
const DEFAULT_KEY_FILE: &'static str = "key.pem";
const KEY_FILE_ENV_VAR: &'static str = "PRIV_KEY";

/// Setup logging at the beginning of the program.
pub fn setup() {
    if std::env::var(LOG_LEVEL_ENV_VAR).is_err() {
        std::env::set_var(LOG_LEVEL_ENV_VAR, DEFAULT_LOG_LEVEL);
    }

    env_logger::init();
    info!("Starting up.");
    info!(
        "Current Working directory: {}",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    info!("Address set: {}", *ADDRESS);
    info!("Client ID: {}", CLIENT_ID);
    info!("Cert file location: {}", *CERT_FILE);
    info!("Private key location: {}", *KEY_FILE);
    info!("Log level set: {}", std::env::var(LOG_LEVEL_ENV_VAR).unwrap());
}

/// Function to get the hosting address.
fn address() -> String {
    std::env::var(ADDRESS_ENV_VAR)
        .unwrap_or(DEFAULT_ADDRESS.to_owned())
}

/// Function to get the cert file location.
fn cert_file() -> String {
    std::env::var(CERT_FILE_ENV_VAR).unwrap_or(DEFAULT_CERT_FILE.to_owned())
}

/// Function to get the private key location.
fn priv_key_file() -> String {
    std::env::var(KEY_FILE_ENV_VAR).unwrap_or(DEFAULT_KEY_FILE.to_owned())
}

/// Function to get client ID.
fn client_id() -> String {
    std::env::var(CLIENT_ID_ENV_VAR).unwrap_or(DEFAULT_CLIENT_ID.to_owned())
}