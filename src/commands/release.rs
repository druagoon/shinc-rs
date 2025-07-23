use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use clap::{Parser, ValueHint};

use crate::prelude::*;
use crate::utils::formatter::identifier;
use crate::utils::path::add_extension;
use crate::utils::process::quit_if;
use crate::utils::terminal::confirm_or_exit;
use crate::utils::tips;
use crate::utils::vcs::git;
use crate::validator::validate_semver;

/// Create a new release
#[derive(clap::Parser, Debug)]
pub struct ReleaseCmd {
    /// Semantic version number
    #[arg(value_hint = ValueHint::Other, value_parser = validate_semver)]
    version: String,
    /// Skip release confirmation
    #[arg(long)]
    no_confirm: bool,
    /// Do not commit changes
    #[arg(long)]
    no_commit: bool,
    /// Do not create a tag
    #[arg(long)]
    no_tag: bool,
    /// Do not push to remote repository
    #[arg(long)]
    no_push: bool,
    /// Git remote to push
    #[arg(long, value_name = "NAME", default_value = "origin")]
    remote_name: String,
}

impl CliCommand for ReleaseCmd {
    fn run(&self) -> CliResult {
        tips::debug(
            "Releasing binaries (bump version, update CHANGELOG, create a tag, push to remote)",
        );

        let current_dir = std::env::current_dir()?;
        let repo = git::Repo::new(&current_dir)?;
        let tag = format!("v{}", self.version);
        if repo.tag_exists(&tag)? {
            tips::die(&format!("The tag '{tag}' already exists"));
        }

        let name = CONFIG.bins().iter().map(|b| b.name()).collect::<Vec<_>>().join(" ");
        let branch = repo.current_branch()?;
        if !self.no_confirm {
            confirm_or_exit(&format!("Release {} {}", identifier(&name), self.version));
            confirm_or_exit(&format!("Branch: {}", identifier(&branch)));
        }

        tips::h1("Git info");
        println!("Version: {}\nTag: {}\nBranch: {}", self.version, tag, branch);
        tips::h1("Updating version");
        let config_path = CONFIG.path();
        update_project_version(&config_path, &self.version)?;

        tips::h1("Updating changelog");
        let changelog = CONFIG.changelog();
        update_changelog(changelog, &tag)?;

        quit_if(self.no_commit);
        tips::h1("Committing changes");
        let files = [PathBuf::from(changelog), relative_path(&current_dir, config_path)?];
        let message = format!("chore: Release {} {}", name, self.version);
        repo.commit(&files, &message)?;

        quit_if(self.no_tag);
        tips::h1("Creating tag");
        repo.create_tag(&tag, &message)?;

        quit_if(self.no_push);
        tips::h1("Pushing commits and tag");
        let output = push_branch_and_tag(current_dir, &self.remote_name, &branch, &tag)?;
        println!("{output}");

        Ok(())
    }
}

fn update_changelog(output: &str, tag_name: &str) -> anyhow::Result<()> {
    let args = ["git-cliff", "--tag", tag_name, "--output", output];
    let opts = git_cliff::args::Opt::try_parse_from(args)?;
    log::debug!("git-cliff opts: {opts:#?}");
    git_cliff::run(opts)?;
    Ok(())
}

pub fn update_project_version<P: AsRef<Path>>(
    config_path: P,
    new_version: &str,
) -> anyhow::Result<()> {
    let content = fs::read_to_string(&config_path)?;
    let mut doc: toml_edit::DocumentMut = content.parse()?;
    doc["project"]["version"] = toml_edit::value(new_version);
    let parsed = doc.to_string();
    atomic_write(config_path, &parsed)?;
    Ok(())
}

fn atomic_write<P: AsRef<Path>>(p: P, data: &str) -> anyhow::Result<()> {
    let path = p.as_ref();
    let temp_path = add_extension(path, "bak")?;
    fs::write(&temp_path, data)?;
    fs::rename(&temp_path, path)?;
    Ok(())
}

fn push_branch_and_tag<P: AsRef<Path>>(
    workdir: P,
    remote_name: &str,
    branch: &str,
    tag: &str,
) -> anyhow::Result<String> {
    let path = workdir.as_ref().to_str().unwrap_or_default();
    let args = vec!["-C", path, "push", remote_name, branch, tag];
    let output = Command::new("git").args(args).stderr(Stdio::piped()).output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stderr).trim().to_string())
    } else {
        anyhow::bail!(
            "An error occurred when push to remote.\n{}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
}

fn relative_path<W, P>(workdir: W, path: P) -> anyhow::Result<PathBuf>
where
    W: AsRef<Path>,
    P: AsRef<Path>,
{
    let relative_path = path.as_ref().strip_prefix(workdir)?;
    Ok(relative_path.to_path_buf())
}
