use commitfmt_macros::rules_enum;

pub mod body;
pub mod header;

#[derive(Copy, Clone, Debug)]
pub enum Linter {
    Header,
    Body,
    Footer,
}

rules_enum! {
    (Body, "leading-newline") => body::LeadingNewLine,
    (Body, "max-line-length") => body::MaxLineLength,
    (Header, "description-leading-space") => header::DescriptionLeadingSpace,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinterParseError;

impl std::str::FromStr for Linter {
    type Err = LinterParseError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(match name {
            "header" => Linter::Header,
            "body" => Linter::Body,
            "footer" => Linter::Footer,
            _ => return Err(LinterParseError),
        })
    }
}

impl Linter {
    pub fn as_display(&self) -> &'static str {
        match self {
            Linter::Header => "header",
            Linter::Body => "body",
            Linter::Footer => "footer",
        }
    }

    pub fn all() -> &'static [Linter] {
        &[Linter::Header, Linter::Body, Linter::Footer]
    }

    pub fn iter() -> impl Iterator<Item = Linter> {
        Linter::all().iter().copied()
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Settings {
    pub body: body::Settings,
    pub header: header::Settings,
}
