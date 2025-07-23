mod git_ops;
use git_ops::GitRepository;

fn main() -> anyhow::Result<()> {
    let repo: GitRepository = GitRepository::new();
    repo.stage_files(files);
    Ok(())
}


