use std::io;
use std::path::PathBuf;

use heck::ToPascalCase;

use crate::include_template;
use crate::prelude::*;
use crate::utils::fs::create_file;
use crate::utils::hashlib::calculate_url_sha256;

const FORMULA_NAME: &str = "homebrew/formula.rb";
const FORMULA_TEMPLATE: &str = include_template!("homebrew/formula.rb");
const TEMPLATES: [(&str, &str); 1] = [(FORMULA_NAME, FORMULA_TEMPLATE)];

/// Generate Homebrew formulae.
#[derive(clap::Parser, Debug)]
pub struct HomebrewFormulaCmd {
    /// Formula name if not specified using inferred name
    #[arg(long)]
    pub name: Option<String>,
    /// Write formula file to directory instead of stdout
    #[arg(long, value_name = "DIR")]
    pub output_dir: Option<PathBuf>,
}

impl HomebrewFormulaCmd {
    fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| CONFIG.inferred_dist_name())
    }

    fn url<R: AsRef<str>>(&self, repository: R, name: &str, version: &str) -> String {
        let repo = repository.as_ref();
        format!("{repo}/releases/download/v{version}/{name}-v{version}.tar.gz")
    }

    fn url_checksum(
        &self,
        repository: Option<&url::Url>,
        name: &str,
        version: &str,
    ) -> (String, String) {
        if let Some(r) = repository {
            let url = self.url(r, name, version);
            let checksum = calculate_url_sha256(&url).unwrap_or_else(|e| {
                log::error!("Failed to calculate sha256 for '{}': {}", url, e);
                String::new()
            });
            (url, checksum)
        } else {
            (String::new(), String::new())
        }
    }

    fn formula_file(&self, name: &str) -> Option<PathBuf> {
        self.output_dir.as_ref().map(|x| x.join(format!("{}.rb", name)))
    }

    fn render_formula(&self, name: &str, ctx: &mut tera::Context) -> anyhow::Result<()> {
        let engine = init_engine()?;
        if let Some(filepath) = self.formula_file(name) {
            let fp = create_file(filepath)?;
            engine.render_to(FORMULA_NAME, ctx, fp)?;
        } else {
            engine.render_to(FORMULA_NAME, ctx, &io::stdout())?;
        }
        Ok(())
    }
}

impl CliCommand for HomebrewFormulaCmd {
    fn run(&self) -> CliResult {
        let name = self.name();
        let project = CONFIG.project();
        let repository = project.repository();
        let (url, checksum) = self.url_checksum(repository, &name, project.version());
        let mut ctx = tera::Context::new();
        ctx.insert("name", &name);
        ctx.insert("classname", &name.to_pascal_case());
        ctx.insert("url", &url);
        ctx.insert("checksum", &checksum);
        ctx.insert("project", project);
        ctx.insert("bins", &CONFIG.bins());
        self.render_formula(&name, &mut ctx)?;
        Ok(())
    }
}

fn init_engine() -> anyhow::Result<tera::Tera> {
    let mut engine = tera::Tera::default();
    engine.add_raw_templates(TEMPLATES)?;
    Ok(engine)
}
