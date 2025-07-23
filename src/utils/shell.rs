use std::path::Path;
use std::process::{Command, Stdio};

use which::which;

use crate::prelude::*;

#[allow(dead_code)]
pub fn fmt_shell<P: AsRef<Path>>(p: P) -> anyhow::Result<()> {
    if let Ok(cmd) = which("shfmt") {
        let output = Command::new(cmd)
            .args(CONFIG.shfmt_options())
            .arg(p.as_ref())
            .stderr(Stdio::piped())
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    anyhow::bail!(
                        "shfmt failed with error:\n{}",
                        String::from_utf8_lossy(&output.stderr)
                    )
                } else {
                    Ok(())
                }
            }
            Err(err) => {
                anyhow::bail!("failed to execute shfmt: {err}")
            }
        }
    } else {
        anyhow::bail!("shfmt is not installed or not in PATH.")
    }
}
