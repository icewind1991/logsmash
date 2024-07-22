mod server_24;
mod server_25;
mod server_26;
mod server_27;
mod server_28;
mod server_29;

pub const MIN_VERSION: u32 = 24;
pub const MAX_VERSION: u32 = 29;

pub fn get_statements(name: &str, version: u32) -> &[crate::LoggingStatement] {
    match (name, version) {
        ("server", 24) => server_24::STATEMENTS,
        ("server", 25) => server_25::STATEMENTS,
        ("server", 26) => server_26::STATEMENTS,
        ("server", 27) => server_27::STATEMENTS,
        ("server", 28) => server_28::STATEMENTS,
        ("server", 29) => server_29::STATEMENTS,
        _ => &[],
    }
}
