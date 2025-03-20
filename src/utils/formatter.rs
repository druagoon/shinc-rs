use std::fmt::Display;

use colored::{Color, ColoredString, Colorize};

#[derive(Clone, Debug, Default)]
pub struct Formatter {
    indent: Option<String>,
    prefix: Option<String>,
    text: Option<String>,
    color: Option<Color>,
}

#[allow(dead_code)]
impl Formatter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn indent(mut self, indent: &str) -> Self {
        self.indent = Some(indent.to_string());
        self
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn arrow(self) -> Self {
        self.prefix("==>")
    }

    pub fn headline(self, text: &str) -> Self {
        self.arrow().text(&text.bold())
    }

    pub fn label(self, label: &str) -> Self {
        self.prefix(&format!("{}:", label))
    }

    pub fn success(self, label: &str) -> Self {
        self.label(label).color(Color::Green)
    }

    pub fn warning(self, label: &str) -> Self {
        self.label(label).color(Color::Yellow)
    }

    pub fn error(self, label: &str) -> Self {
        self.label(label).color(Color::Red)
    }
}

impl Display for Formatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent = self.indent.as_deref().unwrap_or("");
        let prefix = self.prefix.as_deref().unwrap_or("");
        let text = self.text.as_deref().unwrap_or("");
        let r = match (prefix, self.color) {
            ("", Some(c)) => format!("{}{}", indent, text.color(c)),
            ("", None) => format!("{}{}", indent, text),
            (_, Some(c)) => format!("{}{} {}", indent, prefix.color(c), text),
            (_, None) => format!("{}{} {}", indent, prefix, text),
        };
        write!(f, "{}", r)
    }
}

#[allow(dead_code)]
pub fn url(url: &str) -> ColoredString {
    url.underline()
}

#[allow(dead_code)]
pub fn identifier(ident: &str) -> ColoredString {
    ident.green()
}
