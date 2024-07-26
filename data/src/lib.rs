mod data;
mod types;

pub use data::*;
use std::borrow::Cow;
pub use types::*;
use version_compare::{compare, Cmp};

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

pub struct SourceDefinition<'a> {
    pub name: &'a str,
    pub version: Cow<'a, str>,
}

fn try_get_statements_for(
    name: &str,
    version: &str,
) -> Option<(&'static str, &'static [LoggingStatement])> {
    let min_version = MIN_VERSIONS
        .iter()
        .copied()
        .find_map(|(min_name, version)| (min_name == name).then_some(version))?;
    let max_version = MAX_VERSIONS
        .iter()
        .copied()
        .find_map(|(min_name, version)| (min_name == name).then_some(version))?;

    let mut version = if compare(version, min_version) == Ok(Cmp::Lt) {
        min_version
    } else if compare(version, max_version) == Ok(Cmp::Gt) {
        max_version
    } else {
        version
    };

    loop {
        if let Some(result) = get_statements_for(name, version) {
            return Some(result);
        }
        if let Some((looser, _)) = version.rsplit_once('.') {
            version = looser;
        } else {
            return None;
        }
    }
}

pub fn default_apps() -> impl Iterator<Item = SourceDefinition<'static>> {
    MAX_VERSIONS.iter().copied().filter_map(|(name, version)| {
        (name != "server").then_some(SourceDefinition {
            name,
            version: version.into(),
        })
    })
}

pub fn get_statements<'a, I: Iterator<Item = SourceDefinition<'a>>>(sources: I) -> StatementList {
    StatementList::new(
        sources
            .into_iter()
            .flat_map(|def| try_get_statements_for(def.name, def.version.as_ref()))
            .collect(),
    )
}
