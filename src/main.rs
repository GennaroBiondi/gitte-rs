mod branch;
mod commit;

use anyhow::{Context, Result, anyhow};
use branch::GitBranch;
use colored::Colorize;
use commit::GitCommit;
use git2::Repository;
use std::{
    env::{self, args},
    path::PathBuf,
};

fn check_commits(repo: &Repository) -> Result<()> {
    let mut rev_walk = repo.revwalk()?;
    rev_walk.push_head()?;

    for oid in rev_walk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let short_oid = &oid.to_string()[..7];
        let summary = commit.summary()?;

        match summary {
            Some(s) => {
                println!(
                    "{}",
                    format!("  Checking Commit '{}' [{}]", s, &short_oid).purple()
                );
                match GitCommit::new(s.to_string()) {
                    Err(err) => eprintln!("    {}", err.to_string().red()),
                    _ => {}
                }
            }
            None => {
                println!(
                    "{}",
                    format!("  Checking Commit 'empty' [{}]", &short_oid).purple()
                );
                eprintln!("    {}", "Commit summary is empty!".red());
            }
        }
    }

    Ok(())
}

fn check_branches(repo: &Repository) -> Result<()> {
    let branches = repo.branches(None)?;

    for b in branches {
        let b = b?;
        let branch_name =
            b.0.name()?
                .ok_or_else(|| anyhow!("  Branch name is no valid utf-8!"))?;

        println!(
            "{}",
            format!("  Checking Branch '{}'", branch_name).purple()
        );

        let branch = GitBranch::new(branch_name.to_string());

        match branch {
            Err(err) => eprintln!("    {}", err.to_string().red()),
            _ => {}
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let repo_path = if args.len() < 2 {
        PathBuf::from(".")
    } else {
        let path_str = &args[1];
        PathBuf::from(PathBuf::from(path_str))
    };

    let current_repo =
        Repository::open(repo_path).context("No Repository found in current directory!")?;

    println!("{}", "Checking all Repo Commits...".purple());
    check_commits(&current_repo)?;

    println!("{}", "Checking all Repo Branches...".purple());
    check_branches(&current_repo)?;

    Ok(())
}
