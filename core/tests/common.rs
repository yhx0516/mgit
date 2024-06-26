use lazy_static::lazy_static;
use mgit::utils::progress::{Progress, RepoInfo};
use mgit::utils::style_message::StyleMessage;
use std::path::Path;
use std::process::Stdio;

#[allow(unused)]
pub const DEFAULT_BRANCH: &str = "master";

#[allow(unused)]
pub mod failed_message {
    pub const GIT_INIT: &str = "git init failed";
    pub const GIT_ADD_REMOTE: &str = "git add remote failed";
    pub const GIT_STAGE: &str = "git stage failed";
    pub const GIT_COMMIT: &str = "git commit failed";
    pub const GIT_STATUS: &str = "git status failed";
    pub const GIT_CHECKOUT: &str = "git checkout failed";
    pub const GIT_RESET: &str = "git reset failed";
    pub const GIT_STASH_LIST: &str = "git stash list failed";
    pub const GIT_STASH_POP: &str = "git stash pop failed";
    pub const GIT_BRANCH: &str = "git branch failed";
    pub const GIT_FETCH: &str = "git fetch failed";
    pub const GIT_CONFIG: &str = "git config failed";
    pub const GIT_REV_LIST: &str = "git rev-list failed";
    pub const GIT_SPARSE_CHECKOUT: &str = "git sparse-checkout failed";

    pub const WRITE_FILE: &str = "write file failed";
}

pub fn exec_cmd(path: impl AsRef<Path>, cmd: &str, args: &[&str]) -> Result<String, anyhow::Error> {
    let output = std::process::Command::new(cmd)
        .current_dir(path.as_ref())
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    match output.status.success() {
        false => Err(anyhow::anyhow!(stderr)),
        true => Ok(stdout),
    }
}

lazy_static! {
    static ref USE_GITEA: bool = use_gitea();
    pub static ref MGIT_REPO: &'static str = match &USE_GITEA as &bool {
        true => "http://localhost:3000/mgit/mgit.git",
        false => "https://github.com/funny/mgit.git",
    };
    pub static ref IMGUI_REPO: &'static str = match &USE_GITEA as &bool {
        true => "http://localhost:3000/mgit/imgui-rs.git",
        false => "https://github.com/imgui-rs/imgui-rs.git",
    };
    pub static ref SBERT_REPO: &'static str = match &USE_GITEA as &bool {
        true => "http://localhost:3000/mgit/rust-sbert.git",
        false => "https://gitee.com/icze1i0n/rust-sbert.git",
    };
    pub static ref CSBOOKS_REPO: &'static str = match &USE_GITEA as &bool {
        true => "http://localhost:3000/mgit/CS-Books.git",
        false => "https://gitee.com/ForthEspada/CS-Books.git",
    };
}

pub struct TomlBuilder {
    toml_string: String,
}

impl Default for TomlBuilder {
    fn default() -> Self {
        TomlBuilder {
            toml_string:
                "# This file is automatically @generated by mgit.\n# Editing it as you wish.\n"
                    .to_string(),
        }
    }
}

impl TomlBuilder {
    pub fn build(self) -> String {
        self.toml_string
    }

    pub fn default_branch(mut self, default_branch: impl AsRef<str>) -> Self {
        self.toml_string.push_str(&format!(
            "default-branch = \"{}\"\n",
            default_branch.as_ref()
        ));
        self
    }

    pub fn join_repo(
        mut self,
        local: &str,
        remote: &str,
        branch: Option<&str>,
        commit: Option<&str>,
        tag: Option<&str>,
    ) -> Self {
        self.toml_string
            .push_str(&format!("\n[[repos]]\nlocal = \"{}\"\n", local));
        self.toml_string
            .push_str(&format!("remote = \"{}\"\n", remote));
        if let Some(branch) = branch {
            self.toml_string
                .push_str(&format!("branch = \"{}\"\n", branch));
        }
        if let Some(commit) = commit {
            self.toml_string
                .push_str(&format!("commit = \"{}\"\n", commit));
        }
        if let Some(tag) = tag {
            self.toml_string.push_str(&format!("tag = \"{}\"\n", tag));
        }
        self
    }
}

fn use_gitea() -> bool {
    cfg!(feature = "use_gitea")
}

#[derive(Clone, Default)]
pub struct TestProgress;

#[allow(unused)]
impl Progress for TestProgress {
    fn repos_start(&self, _total: usize) {}

    fn repos_end(&self) {}

    fn repo_start(&self, repo_info: &RepoInfo, message: StyleMessage) {}

    fn repo_info(&self, repo_info: &RepoInfo, message: StyleMessage) {}

    fn repo_end(&self, repo_info: &RepoInfo, message: StyleMessage) {}

    fn repo_error(&self, repo_info: &RepoInfo, message: StyleMessage) {}
}
