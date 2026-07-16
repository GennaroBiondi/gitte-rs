use anyhow::{Error, Result, anyhow, bail};
use std::str::FromStr;

#[derive(Debug)]
pub enum CommitType {
    Fix,
    Feat,
    Build,
    Chore,
    Ci,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
}

impl FromStr for CommitType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fix" => Ok(Self::Fix),
            "feat" => Ok(Self::Feat),
            "build" => Ok(Self::Build),
            "chore" => Ok(Self::Chore),
            "ci" => Ok(Self::Ci),
            "docs" => Ok(Self::Docs),
            "style" => Ok(Self::Style),
            "refactor" => Ok(Self::Refactor),
            "perf" => Ok(Self::Perf),
            "test" => Ok(Self::Test),
            _ => bail!("Commit Type not recognized: '{}'", s),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct GitCommit {
    commit_type: CommitType,
    scope: Option<String>,
    description: String,
    raw_name: String,
}

impl GitCommit {
    pub fn new(commit_name: String) -> Result<Self> {
        let raw_name = commit_name;

        let open_paren = raw_name.find('(');
        let close_paren = raw_name.find(')');

        let scope = match (open_paren, close_paren) {
            (Some(o), Some(c)) => Some(raw_name[o + 1..c].to_string()),
            (None, None) => None,
            _ => bail!("Mismatched parentheses in scope!"),
        };

        if let Some(ref x) = scope {
            if !x.is_ascii() {
                bail!("Scope is not valid ascii!");
            }

            if x.contains(" \t\n") {
                bail!("Scope contains either a space, tab, or new line character!");
            }

            if x.contains("():@") {
                bail!("Scope contains invalid character");
            }
        }

        let commit_type_end = open_paren
            .or(raw_name.find(':'))
            .ok_or_else(|| anyhow!("No scope or colon found!"))?;

        let raw_type = &raw_name[..commit_type_end];
        let commit_type_str = raw_type.strip_suffix("!").unwrap_or(raw_type);
        let commit_type: CommitType = commit_type_str.parse()?;

        let desc_start = raw_name
            .find(": ")
            .or_else(|| raw_name.find("): ").map(|x| x + 1))
            .ok_or_else(|| anyhow!("No description found!"))?;

        let description = raw_name[desc_start + 2..].to_string();

        Ok(Self {
            commit_type,
            scope,
            description,
            raw_name,
        })
    }
}
