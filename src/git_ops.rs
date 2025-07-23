use git2;
pub struct GitRepository {
    path: String,
    repo: git2::Repository
}

impl GitRepository {
    
    pub fn new() -> Self {
        let repo: git2::Repository = git2::Repository::discover(".").expect("Failed to find repository");
        let path: String = repo.path().to_str().unwrap_or("").to_string();
        GitRepository { path: path, repo: repo }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn repo(&self) -> &git2::Repository {
        &self.repo
    }

    pub fn files_unstaged(&self) -> Result<Vec<String>, git2::Error> {
        let mut untracked_files = Vec::new();
        let statuses = self.repo.statuses(None)?;
        for status in statuses.iter() {
            if status.status().is_wt_new() || status.status().is_wt_modified() {
                if let Some(path) = status.path() {
                    untracked_files.push(path.to_string());
                }
            }
        }
        Ok(untracked_files)
    }
    
    pub fn stage_files(&self, files: &[&str]) -> Result<(), git2::Error> {
        let mut index = self.repo.index()?;
        for file in files {
            index.add_path(std::path::Path::new(file))?;
        }
        index.write()?;
        Ok(())
    }   
}