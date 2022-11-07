use super::{display_path, execute_cmd, load_config, TomlRepo};

use git2::Repository;
use owo_colors::OwoColorize;
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn exec(path: Option<String>, config: Option<PathBuf>) {
    let cwd = env::current_dir().unwrap();
    let cwd_str = Some(String::from(cwd.to_string_lossy()));
    let input = path.or(cwd_str).unwrap();
    let input_path = Path::new(&input);

    // if directory doesn't exist, finsh clean
    if input_path.is_dir() == false {
        println!("Directory {} not found!", input.bold().magenta());
        return;
    }

    // start set track remote branch
    println!("Track status:");

    // set config file path
    let config_file = match config {
        Some(r) => r,
        _ => input_path.join(".gitrepos"),
    };

    // check if .gitrepos exists
    if config_file.is_file() == false {
        println!(
            "{} not found, try {} instead!",
            ".gitrepos".bold().magenta(),
            "init".bold().magenta()
        );
        return;
    }

    // load .gitrepos
    if let Some(toml_config) = load_config(&config_file) {
        let default_branch = toml_config.default_branch;

        // handle sync
        if let Some(toml_repos) = toml_config.repos {
            for toml_repo in toml_repos {
                if let Ok(res) = set_tracking_remote_branch(input_path, &toml_repo, &default_branch)
                {
                    println!("  {}", res);
                }
            }
        }
    }
}

pub fn set_tracking_remote_branch(
    input_path: &Path,
    toml_repo: &TomlRepo,
    default_branch: &Option<String>,
) -> Result<String, anyhow::Error> {
    let rel_path = toml_repo.local.as_ref().unwrap();
    let full_path = input_path.join(rel_path);

    // try open git repo
    let repo_result = Repository::open(&full_path);
    let mut local_branch = String::new();

    if let Ok(repo) = repo_result {
        if let Ok(refname) = repo.head() {
            local_branch = refname.shorthand().unwrap().to_string();
        }
    } else {
        let res = format!(
            "{}: {}",
            display_path(rel_path).magenta(),
            "repository doesn't exist".red()
        );
        return Ok(res);
    }

    // priority: commit/tag/branch/default-branch
    let remote_head = {
        if let Some(commit) = &toml_repo.commit {
            (&commit[..7]).to_string()
        } else if let Some(tag) = &toml_repo.tag {
            tag.to_string()
        } else if let Some(branch) = &toml_repo.branch {
            "origin/".to_string() + &branch.to_string()
        } else if let Some(branch) = default_branch {
            "origin/".to_string() + &branch.to_string()
        } else {
            String::new()
        }
    };

    if toml_repo.commit.is_some() || toml_repo.tag.is_some() {
        let res = format!(
            "{}: {} {}",
            display_path(rel_path).magenta(),
            remote_head.blue(),
            "untracked"
        );
        return Ok(res);
    }

    // git branch --set-upstream-to <name>
    // true only when remote head is branch
    let args = vec!["branch", "--set-upstream-to", &remote_head];
    if execute_cmd(&full_path, "git", &args).is_ok() {
        let res = format!(
            "{}: {} -> {}",
            display_path(rel_path).magenta(),
            local_branch.blue(),
            remote_head.blue()
        );
        Ok(res)
    } else {
        let res = format!(
            "{}: {} {} {}",
            display_path(rel_path).magenta(),
            "track failed,".red(),
            remote_head.blue(),
            "not found!".red()
        );
        Ok(res)
    }
}