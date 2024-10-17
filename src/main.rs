use git2::Repository;
use open;
use toml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    default_branch: String,
    owner: String,
    repo_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // find git project root
    let repo = Repository::discover(".")?;
    let repo_path = repo.path().parent().unwrap().to_str().unwrap();
    let mut default_branch = String::new();
    let mut owner = String::new();
    let mut repo_name = String::new();



    // Check if config already defines owner and repo_name
    let config_path = format!("{}/.git/open_pr.toml", repo_path);
    if std::fs::metadata(&config_path).is_ok() {
        let config_str = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&config_str)?;
        owner = config.owner;
        repo_name = config.repo_name;
        println!("Owner: {}", owner);
        println!("Repository Name: {}", repo_name);
        println!("Default Branch: {}", default_branch);
        println!("You can tweak this configuration in .git/open_pr.toml");
    }
    else {
        // prompt owner and repo_name
        println!("Enter the owner of the repository (The \"org\" in github.com/org/reponame): ");
        std::io::stdin().read_line(&mut owner).unwrap();
        owner = owner.trim().to_string();
        println!("Enter the repository name: (The \"reponame\" in github.com/org/reponame): ");
        std::io::stdin().read_line(&mut repo_name).unwrap();
        repo_name = repo_name.trim().to_string();
        println!("Enter the default branch (usually main or master): ");
        std::io::stdin().read_line(&mut default_branch).unwrap();
        default_branch = default_branch.trim().to_string();
        println!("You can tweak this configuration later in .git/open_pr.toml");
    }

    // Write to .open_pr.toml
    let config: Config = Config {
        owner: owner.clone(),
        repo_name: repo_name.clone(),
        default_branch: default_branch.clone(),
    };
    let config_str = toml::to_string(&config).unwrap();
    std::fs::write(format!("{}/.git/open_pr.toml", repo_path), config_str)?;

    let repo = Repository::open(".")?;

    // Get the current branch
    let head = repo.head()?;
    let head_ref = head.shorthand().unwrap_or("unknown");
    let base_branch = default_branch;

    // Build the URL for creating a pull request on GitHub
    let pr_url = format!(
        "https://github.com/{}/{}/compare/{}...{}?expand=1",
        owner, repo_name, base_branch, head_ref
    );

    // Print the URL (optional)
    println!("Opening PR URL: {}", pr_url);

    // Open the URL in the default web browser
    open::that(pr_url)?;

    Ok(())
}
