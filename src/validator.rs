pub fn validate_semver(val: &str) -> anyhow::Result<String> {
    match semver::Version::parse(val) {
        Ok(_) => Ok(val.to_string()),
        Err(_) => Err(anyhow::format_err!("need semantic version number")),
    }
}
