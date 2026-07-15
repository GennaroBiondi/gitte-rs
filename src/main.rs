mod branch;
mod commit;

use anyhow::Result;
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
                println!("Checking Commit '{}' [{}]", s, &short_oid);
                match GitCommit::new(s.to_string()) {
                    Err(err) => eprintln!("{}", err),
                    _ => {}
                }
            }
            None => {
                println!("Checking Commit 'empty' [{}]", &short_oid);
                eprintln!("Commit summary is empty!");
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let current_repo = Repository::open(".")?;
    check_commits(&current_repo)?;

    Ok(())
}
