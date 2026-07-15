mod branch;
mod commit;

use anyhow::{Result, anyhow};
use branch::GitBranch;
use commit::GitCommit;
use git2::Repository;

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
                println!("  Checking Commit '{}' [{}]", s, &short_oid);
                match GitCommit::new(s.to_string()) {
                    Err(err) => eprintln!("    {}", err),
                    _ => {}
                }
            }
            None => {
                println!("  Checking Commit 'empty' [{}]", &short_oid);
                eprintln!("    Commit summary is empty!");
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

        println!("  Checking Branch '{}'", branch_name);

        let branch = GitBranch::new(branch_name.to_string());

        match branch {
            Err(err) => eprintln!("    {}", err),
            _ => {}
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let current_repo = Repository::open(".")?;

    println!("Checking all Repo Commits...");
    check_commits(&current_repo)?;

    println!("Checking all Repo Branches...");
    check_branches(&current_repo)?;

    Ok(())
}
