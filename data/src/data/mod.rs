use crate::LoggingStatement;

mod deck_1_13;
mod files_accesscontrol_1_19;
mod files_antivirus_5_5;
mod server_29;

pub const MIN_VERSIONS: &[(&str, &str)] = &[
      ("deck", "1.13"),
("files_accesscontrol", "1.19"),
("files_antivirus", "5.5"),
("server", "29"),
    ];

pub const MAX_VERSIONS: &[(&str, &str)] = &[
  ("deck", "1.13"),
("files_accesscontrol", "1.19"),
("files_antivirus", "5.5"),
("server", "29"),
];

pub fn get_statements_for(name: &str, version: &str) -> Option<(&'static str, &'static [LoggingStatement])> {
    match (name, version) {
        ("deck", "1.13") => Some(("/apps/deck", deck_1_13::STATEMENTS)),
("files_accesscontrol", "1.19") => Some(("/apps/files_accesscontrol", files_accesscontrol_1_19::STATEMENTS)),
("files_antivirus", "5.5") => Some(("/apps/files_antivirus", files_antivirus_5_5::STATEMENTS)),
("server", "29") => Some(("", server_29::STATEMENTS)),
        _ => None,
    }
}
