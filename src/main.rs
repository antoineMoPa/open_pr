use core::fmt;
use git2::Repository;
use open;
use serde::{Deserialize, Serialize};
use std::io;
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    default_branch: String,
    owner: String,
    repo_name: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Owner: {}\nRepository Name: {}\nDefault Branch: {}",
            self.owner, self.repo_name, self.default_branch
        )
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // find git project root
    let repo = Repository::discover(".")?;
    let repo_path = repo.path().parent().unwrap().to_str().unwrap();

    // Use match to either load the config from the file or prompt the user
    let config_path = format!("{}/.git/open_pr.toml", repo_path);
    let config = match std::fs::read_to_string(&config_path) {
        Ok(config_str) => {
            let config: Config = toml::from_str(&config_str)?;
            println!("{}", config);
            println!("You can tweak this configuration in .git/open_pr.toml");
            config
        }
        Err(_) => {
            println!(
                "Enter the owner of the repository (The \"org\" in github.com/org/reponame): "
            );
            let owner = get_user_input();

            println!("Enter the repository name: (The \"reponame\" in github.com/org/reponame): ");
            let repo_name = get_user_input();

            println!("Enter the default branch (usually main or master): ");
            let default_branch = get_user_input();

            let config = Config {
                owner: owner.trim().to_string(),
                repo_name: repo_name.trim().to_string(),
                default_branch: default_branch.trim().to_string(),
            };

            println!("You can tweak this configuration later in .git/open_pr.toml");

            // Write the new configuration to the file
            let config_str = toml::to_string(&config).unwrap();
            std::fs::write(&config_path, config_str)?;

            config
        }
    };

    let repo = Repository::open(".")?;

    // Get the current branch
    let head = repo.head()?;
    let head_ref = head.shorthand().unwrap_or("unknown");
    let base_branch = &config.default_branch;

    // Build the URL for creating a pull request on GitHub
    let pr_url = format!(
        "https://github.com/{}/{}/compare/{}...{}?expand=1",
        config.owner, config.repo_name, base_branch, head_ref
    );

    // Print the URL (optional)
    println!("Opening PR URL: {}", pr_url);

    // Open the URL in the default web browser
    open::that(pr_url)?;

    Ok(())
}
