use std::io::Write;

use anyhow::Context;

use crate::include_template;
use crate::prelude::*;
use crate::utils::fs::{create_file, set_executable};
use crate::utils::shell::fmt_shell;

const INSTALL_NAME: &str = "shell/install.sh";
const INSTALL_TEMPLATE: &str = include_template!("shell/install.sh");
const TEMPLATES: [(&str, &str); 1] = [(INSTALL_NAME, INSTALL_TEMPLATE)];

/// Generate the install shell script for the project.
#[derive(clap::Parser, Debug)]
pub struct InstallShellCmd {
    /// Filename for the generated install script
    #[arg(short, long, default_value = "install.sh")]
    pub filename: String,
    /// Generate raw script without any processing
    #[arg(long)]
    pub raw: bool,
}

impl InstallShellCmd {
    fn init_engine(&self) -> anyhow::Result<tera::Tera> {
        let mut engine = tera::Tera::default();
        engine.add_raw_templates(TEMPLATES)?;
        Ok(engine)
    }
}

impl CliCommand for InstallShellCmd {
    fn run(&self) -> CliResult {
        let name = CONFIG.inferred_dist_name();
        let url = CONFIG
            .project()
            .repository()
            .map(|u| u.as_str())
            .ok_or_else(|| anyhow::format_err!("Repository URL not set in configuration"))?;
        let mut ctx = tera::Context::new();
        ctx.insert("name", &name);
        ctx.insert("url", url);
        let engine = self.init_engine()?;
        let current_dir = std::env::current_dir()?;
        let target = current_dir.join(&self.filename);
        let fp = create_file(&target)?;
        if self.raw {
            engine.render_to(INSTALL_NAME, &ctx, fp)?;
        } else {
            let source = engine.render(INSTALL_NAME, &ctx)?;
            let content = argc::build(&source, &self.filename, None)?;
            create_file(&target)?
                .write_all(content.as_bytes())
                .with_context(|| format!("Failed to write script to '{}'", target.display()))?;
            set_executable(&target).with_context(|| {
                format!("Failed to set execute permission to '{}'", target.display())
            })?;
            fmt_shell(target)?;
        }
        Ok(())
    }
}
