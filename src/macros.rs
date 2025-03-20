#[macro_export]
macro_rules! template_name {
    ($name:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/templates/", $name)
    };
}

#[macro_export]
macro_rules! include_template {
    ($name:literal) => {
        include_str!($crate::template_name!($name))
    };
}

#[macro_export]
macro_rules! git_ref_branch {
    ($name:expr) => {
        format!("refs/heads/{}", $name)
    };
    ($($name:expr),+) => {
        [$(format!("refs/heads/", $name)),+]
    };
}

#[macro_export]
macro_rules! git_ref_tag {
    ($name:expr) => {
        format!("refs/tags/{}", $name)
    };
    ($($name:expr),+) => {
        [$(format!("refs/tags/", $name)),+]
    };
}

#[macro_export]
macro_rules! git_refs {
    (branch: $name:expr) => {
        [$crate::git_ref_branch!($name)]
    };
    (tag: $name:expr) => {
        [$crate::git_ref_tag!($name)]
    };
    (branch: $branch_name:expr,tag: $tag_name:expr) => {
        [$crate::git_ref_branch!($branch_name), $crate::git_ref_tag!($tag_name)]
    };
}
