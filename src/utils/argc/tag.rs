pub const DESCRIBE: &str = "describe";
pub const META: &str = "meta";
pub const META_VERSION: &str = "version";
pub const META_AUTHOR: &str = "author";
pub const META_DOTENV: &str = "dotenv";
pub const META_DEFAULT_SUBCOMMAND: &str = "default-subcommand";
pub const META_INHERIT_FLAG_OPTIONS: &str = "inherit-flag-options";
pub const META_SYMBOL: &str = "symbol";
pub const META_COMBINE_SHORTS: &str = "combine-shorts";
pub const META_MAN_SECTION: &str = "man-section";
pub const META_REQUIRE_TOOLS: &str = "require-tools";

// Custom tag
pub const INCLUDE: &str = "include";

pub struct ArgcTag;

#[allow(dead_code)]
impl ArgcTag {
    pub fn is_meta_version(version: &str) -> bool {
        version == META_VERSION
    }

    pub fn describe(description: &str) -> String {
        Self::format(DESCRIBE, "", description)
    }

    pub fn meta_version(version: &str) -> String {
        Self::meta(META_VERSION, version)
    }

    pub fn meta_author(author: &str) -> String {
        Self::meta(META_AUTHOR, author)
    }

    pub fn meta_dotenv(dotenv: &str) -> String {
        Self::meta(META_DOTENV, dotenv)
    }

    pub fn meta_default_subcommand(cmd: &str) -> String {
        Self::meta(META_DEFAULT_SUBCOMMAND, cmd)
    }

    pub fn meta_require_tools(require_tools: &[String]) -> String {
        Self::meta(META_REQUIRE_TOOLS, &require_tools.join(","))
    }

    pub fn meta_man_section(man_section: u8) -> String {
        Self::meta(META_MAN_SECTION, &man_section.to_string())
    }

    pub fn meta_inherit_flag_options() -> String {
        Self::meta(META_INHERIT_FLAG_OPTIONS, "")
    }

    pub fn meta_combine_shorts() -> String {
        Self::meta(META_COMBINE_SHORTS, "")
    }

    pub fn meta_symbol(symbol: &str) -> String {
        Self::meta(META_SYMBOL, symbol)
    }

    pub fn meta(name: &str, value: &str) -> String {
        Self::format(META, name, value)
    }

    pub fn format(key: &str, name: &str, value: &str) -> String {
        let mut buf = vec![key, name, value];
        buf.retain(|&x| !x.is_empty());
        format!("# @{}", buf.join(" "))
    }
}
