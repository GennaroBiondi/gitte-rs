use anyhow::{Error, Result, anyhow, bail};
use std::str::FromStr;

#[derive(Debug)]
pub enum BranchType {
    Fix,
    Bugfix,
    Hotfix,
    Feat,
    Release,
    Chore,
    Ai,
    Claude,
    Codex,
    Copilot,
    Cursor,

    Trunk,
    Master,
    Develop,
}

impl BranchType {
    pub fn is_trunk(&self) -> bool {
        matches!(self, Self::Trunk | Self::Master | Self::Develop)
    }
}

impl FromStr for BranchType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fix" => Ok(Self::Fix),
            "bugfix" => Ok(Self::Bugfix),
            "hotfix" => Ok(Self::Hotfix),
            "feat" | "feature" => Ok(Self::Feat),
            "release" => Ok(Self::Release),
            "chore" => Ok(Self::Chore),
            "ai" => Ok(Self::Ai),
            "claude" => Ok(Self::Claude),
            "codex" => Ok(Self::Codex),
            "copilot" => Ok(Self::Copilot),
            "cursor" => Ok(Self::Cursor),

            "main" => Ok(Self::Trunk),
            "master" => Ok(Self::Master),
            "develop" => Ok(Self::Develop),

            _ => bail!("Invalid Branch Type: '{}'", s),
        }
    }
}

#[derive(Debug)]
pub struct GitBranch {
    branch_type: BranchType,
    description: String,
    raw_name: String,
}

impl GitBranch {
    pub fn new(branch_name: String) -> Result<Self> {
        // if raw_name in ("main", "master", "develop"):
        if matches!(branch_name.as_str(), "main" | "master" | "develop") {
            return Ok(Self {
                branch_type: branch_name.parse()?,
                description: String::new(),
                raw_name: branch_name,
            });
        }

        if branch_name.chars().any(|x| x.is_uppercase()) {
            bail!("Found uppercase character in branch name!");
        }

        let slash_separator_index = branch_name
            .find('/')
            .ok_or_else(|| anyhow!("No '/' separator found in branch name!"))?;

        let branch_type_str = &branch_name[0..slash_separator_index];

        if branch_type_str.is_empty() {
            bail!("No type found in branch name!");
        }

        let branch_type: BranchType = branch_type_str.parse()?;

        let description = &branch_name[slash_separator_index + 1..];

        if description.is_empty() {
            bail!("No description found in branch name!");
        }

        let description = description.to_string();

        Ok(Self {
            branch_type,
            description,
            raw_name: branch_name,
        })
    }
}
