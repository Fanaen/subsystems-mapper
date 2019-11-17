use crate::config::{AuthConfig, Target};
use crate::git_extraction::git::{
    open_and_update_or_clone_repo, provide_callbacks, reset_to_branch,
};
use git2::Repository;
use std::cmp::max;
use std::path::Path;

mod git;

pub fn extract_subsystems_from_target(target: &Target, auth: Option<&AuthConfig>) {
    let name = get_name_from_url(target.url.as_str());
    let path = format!("data/{}", name);
    let path = Path::new(path.as_str());

    // Prepare the repository for extraction
    let callbacks = provide_callbacks(auth);
    let repo: Repository = open_and_update_or_clone_repo(target.url.as_str(), path, callbacks);
    reset_to_branch(target.branch.as_ref(), &repo);
}

/// Transforms https://github.com/alexcrichton/git2-rs.git into git2-rs
pub fn get_name_from_url(url: &str) -> &str {
    let last_slash = max(url.rfind('\\'), url.rfind('/'))
        .map(|m| m + 1)
        .unwrap_or(0);
    let length_without_prefix = url.len()
        - if url.ends_with(".git") {
            4usize
        } else {
            0usize
        };

    &url[last_slash..length_without_prefix]
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_name_from_url_https() {
        assert_eq!(
            get_name_from_url("https://github.com/alexcrichton/git2-rs"),
            "git2-rs"
        );
    }

    #[test]
    fn test_get_name_from_url_https_git() {
        assert_eq!(
            get_name_from_url("https://github.com/alexcrichton/git2-rs.git"),
            "git2-rs"
        );
    }

    #[test]
    fn test_get_name_from_url_ssh() {
        assert_eq!(
            get_name_from_url("git@github.com:alexcrichton/git2-rs.git"),
            "git2-rs"
        );
    }
}