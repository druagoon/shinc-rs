use std::io::Write;
use std::path::Path;

use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;

use crate::prelude::*;
use crate::utils::fs::create_file;
use crate::utils::hashlib::calculate_file_sha256;
use crate::utils::tips;

/// Creates a distribution archive.
#[derive(clap::Parser, Debug)]
pub struct DistCmd {}

impl CliCommand for DistCmd {
    fn run(&self) -> CliResult {
        tips::debug("Distributing binaries");

        let name = CONFIG.inferred_dist_name();
        let filename = format!("{}-v{}.tar.gz", name, CONFIG.project().version());
        let output = CONFIG.dist_file(&filename);
        let mut sources = vec![CONFIG.bin_dir(), CONFIG.share_dir()];
        sources.extend(CONFIG.dist_include_extra_paths());
        tips::h1("Archiving files");
        println!("{}", output.display());
        create_archive(&output, &sources)?;

        let checksum = calculate_file_sha256(output)?;
        let checksum_file = CONFIG.dist_file(&format!("{filename}.sha256"));
        tips::h1("Generating sha256sum");
        println!("{}", checksum_file.display());
        let mut fp = create_file(&checksum_file)?;
        writeln!(fp, "{checksum} {filename}")?;

        Ok(())
    }
}

fn create_archive<O, S>(output: O, sources: &[S]) -> anyhow::Result<()>
where
    O: AsRef<Path>,
    S: AsRef<Path>,
{
    let fp = create_file(output)?;
    let enc = GzEncoder::new(fp, Compression::default());
    let mut tar = Builder::new(enc);

    for src in sources {
        let path = src.as_ref();
        if path.exists() {
            let archive_path = match path.strip_prefix(CONFIG.target_dir()) {
                Ok(p) => p,
                Err(_) => path,
            };
            if path.is_dir() {
                tar.append_dir_all(archive_path, path)?;
            } else {
                tar.append_path_with_name(path, archive_path)?;
            }
        } else {
            log::warn!("{} not found, skipping.", path.display());
        }
    }
    tar.finish()?;
    Ok(())
}
