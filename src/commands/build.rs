use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::Context;
use which::which;

use crate::config::Bin;
use crate::prelude::*;
use crate::utils::argc::parser::{parse, EventData};
use crate::utils::argc::tag::ArgcTag;
use crate::utils::formatter::identifier;
use crate::utils::fs::{create_file, read_lines, set_executable};
use crate::utils::tips;

/// Generate and build shell scripts.
#[derive(clap::Parser, Debug)]
pub struct BuildCmd {}

impl CliCommand for BuildCmd {
    fn run(&self) -> CliResult {
        tips::debug("Compiling source and building binaries");

        for bin in &CONFIG.bins() {
            build(bin)?;
        }

        Ok(())
    }
}

struct BuildFile {
    inner: fs::File,
}

impl BuildFile {
    fn new(fp: fs::File) -> Self {
        Self { inner: fp }
    }

    fn flush(&mut self) -> anyhow::Result<()> {
        self.inner.flush()?;
        Ok(())
    }

    fn writeln(&mut self, buf: &str) -> anyhow::Result<()> {
        Ok(writeln!(self.inner, "{buf}")?)
    }

    fn write_newline(&mut self) -> anyhow::Result<()> {
        Ok(writeln!(self.inner)?)
    }

    fn write_meta(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
        self.writeln(&ArgcTag::meta(key, value))?;
        Ok(())
    }

    fn write_meta_version(&mut self, version: &str) -> anyhow::Result<()> {
        self.writeln(&ArgcTag::meta_version(version))?;
        Ok(())
    }

    fn write_include_file<P: AsRef<Path>>(
        &mut self,
        filename: &str,
        path: P,
    ) -> anyhow::Result<()> {
        self.writeln(&format!("# {filename}"))?;
        for line in read_lines(path)? {
            self.writeln(&line?)?;
        }
        self.write_newline()?;
        Ok(())
    }

    fn write_argc_hook(&mut self) -> anyhow::Result<()> {
        self.writeln("\n")?;
        self.writeln(r#"eval "$(argc --argc-eval "$0" "$@")""#)
    }
}

fn get_term_width() -> Option<usize> {
    std::env::var("TERM_WIDTH").ok().and_then(|v| v.parse().ok())
}

#[allow(dead_code)]
fn ensure_path<P: AsRef<Path>>(path: P) -> anyhow::Result<PathBuf> {
    let p = path.as_ref();
    p.canonicalize().with_context(|| format!("file not found: {}", p.display()))
}

fn check_file<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let p = path.as_ref();
    if !p.exists() {
        anyhow::bail!("file not found: {:?}", p);
    }
    Ok(())
}

fn fmt_shell<P: AsRef<Path>>(p: P) {
    if let Ok(cmd) = which("shfmt") {
        let output = Command::new(cmd)
            .args(CONFIG.shfmt_options())
            .arg(p.as_ref())
            .stderr(Stdio::piped())
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    eprintln!(
                        "shfmt failed with error:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(err) => {
                eprintln!("failed to execute shfmt: {err}");
            }
        }
    } else {
        log::warn!("shfmt is not installed or not in PATH.");
    }
}

fn build(bin: &Bin) -> CliResult {
    let bin_name = bin.name();
    tips::title(&format!("Building {}", identifier(bin_name)));

    let src = bin.path();
    check_file(src)?;
    let dst = CONFIG.build_file(&format!("{bin_name}.sh"));
    let mut bf = BuildFile::new(create_file(&dst)?);
    tips::h1("Compiling source");
    println!("{} -> {}", src.display(), dst.display());
    let source = fs::read_to_string(src)
        .with_context(|| format!("failed to load script at '{}'", src.display()))?;
    let events = parse(&source)?;
    for event in &events {
        match &event.data {
            EventData::Include(filename) => {
                let filepath = CONFIG.resolve_src_path(filename);
                check_file(&filepath)?;
                log::debug!("write include file: {filename}");
                bf.write_include_file(filename, filepath)?;
            }
            EventData::Meta(key, value) => {
                if ArgcTag::is_meta_version(key) {
                    bf.write_meta_version(CONFIG.project().version())?;
                } else {
                    bf.write_meta(key, value)?;
                }
            }
            EventData::Unknown(value) => {
                bf.writeln(value)?;
            }
        }
    }
    bf.write_argc_hook()?;
    bf.flush()?;
    fmt_shell(&dst);

    // Build scripts without `argc` dependency
    let target = CONFIG.bin_file(bin_name);
    tips::h1("Argc building");
    println!("{} -> {}", dst.display(), target.display());
    let source = fs::read_to_string(dst)?;
    let content = argc::build(&source, bin_name, get_term_width())?;
    create_file(&target)?
        .write_all(content.as_bytes())
        .with_context(|| format!("failed to write script to '{}'", target.display()))?;
    fmt_shell(&target);
    set_executable(&target)
        .with_context(|| format!("failed to set execute permission to '{}'", target.display()))?;

    Ok(())
}
