use crate::{LoggingStatement, LoggingStatementWithPathPrefix};

// mod server_24;
// mod server_25;
// mod server_26;
// mod server_27;
// mod server_28;
mod files_accesscontrol_1_19;
mod files_antivirus_5_5;
mod server_29;

pub const MIN_VERSION: u32 = 24;
pub const MAX_VERSION: u32 = 29;

pub fn get_statements(version: u32) -> StatementList {
    let server = match version {
        // ("server", 24) => server_24::STATEMENTS,
        // ("server", 25) => server_25::STATEMENTS,
        // ("server", 26) => server_26::STATEMENTS,
        // ("server", 27) => server_27::STATEMENTS,
        // ("server", 28) => server_28::STATEMENTS,
        29 => server_29::STATEMENTS,
        _ => server_29::STATEMENTS,
    };
    StatementList::new(vec![
        ("", server),
        (
            "/apps/files_accesscontrol",
            files_accesscontrol_1_19::STATEMENTS,
        ),
        ("/apps/files_antivirus", files_antivirus_5_5::STATEMENTS),
    ])
}

pub struct StatementList {
    statements: Vec<(&'static str, &'static [LoggingStatement])>,
}

impl StatementList {
    pub fn new(statements: Vec<(&'static str, &'static [LoggingStatement])>) -> StatementList {
        StatementList { statements }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static LoggingStatement> + '_ {
        self.statements
            .iter()
            .copied()
            .flat_map(|(_, list)| list.iter())
    }

    pub fn get(&self, mut index: usize) -> Option<LoggingStatementWithPathPrefix> {
        for (prefix, list) in &self.statements {
            if index < list.len() {
                return list
                    .get(index)
                    .map(|statement| statement.with_path_prefix(prefix));
            }
            index -= list.len()
        }
        None
    }
}
