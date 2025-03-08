use commitfmt_macros::rules_enum;

pub mod body;
pub mod header;

pub enum Linter {
    Header,
    Body,
    Footer,
}

impl Linter {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "header" => Some(Linter::Header),
            "body" => Some(Linter::Body),
            "footer" => Some(Linter::Footer),
            _ => None,
        }
    }
}

rules_enum! {
    (Body, "leading-newline") => body::LeadingNewLine,
    (Body, "max-line-length") => body::MaxLineLength,

    (Header, "description-leading-space") => header::DescriptionLeadingSpace,
}

pub struct Settings {
    pub body: body::Settings,
    pub header: header::Settings,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            body: body::Settings::default(),
            header: header::Settings::default(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rules() {
        assert!(rule_by_name(Linter::Body, "leading-newline").is_some());
        assert!(rule_by_name(Linter::Body, "unknown").is_none());
    }
}
