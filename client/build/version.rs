use std::{fs, process::Command};
use last_git_commit::{
    LastGitCommit,
    Id
};

pub fn get_version() -> String {

    // Should probably use http://crates.io/crates/git2 here
    let res = Command::new("git").args(&["branch", "--show-current"]).output().unwrap();
    let branch = String::from_utf8_lossy(&res.stdout).trim().to_string();

    let commit = LastGitCommit::new(Some("../"), Some(&branch)).unwrap().id.short();
    let cargo_version = env!("CARGO_PKG_VERSION");

    format!("{}-{}", cargo_version, commit)

}

pub fn write_version(version: &String) {

    fs::write("out/version.txt", version).unwrap();

}
