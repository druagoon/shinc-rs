#[macro_export]
macro_rules! string {
    ($s:expr) => {
        $s.to_string()
    };
}

#[macro_export]
macro_rules! include_template {
    ($name:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/", $name))
    };
}
