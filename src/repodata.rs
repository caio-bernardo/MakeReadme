use inquire::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RepoData {
    project_title: String,
    project_description: String,
    github_username: String,
    repo_name: String,
    license: String,
}

impl RepoData {
    pub fn ask_questions() -> Self {
        Self {
            project_title: Text::new("Project title:").prompt().unwrap(),
            project_description: Text::new("Project description:").prompt().unwrap(),
            github_username: Text::new("Github username:").prompt().unwrap(),
            repo_name: Text::new("Repo name:").prompt().unwrap(),
            license: Text::new("License:").prompt().unwrap(),
        }
    }
}
