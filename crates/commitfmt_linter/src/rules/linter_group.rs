#[derive(Copy, Clone, Debug)]
pub enum LinterGroup {
    Header,
    Body,
    Footer,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinterParseError;

impl LinterGroup {
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "header" => LinterGroup::Header,
            "body" => LinterGroup::Body,
            "footer" => LinterGroup::Footer,
            _ => return  None,
        })
    }

    pub fn as_display(&self) -> &'static str {
        match self {
            LinterGroup::Header => "header",
            LinterGroup::Body => "body",
            LinterGroup::Footer => "footer",
        }
    }

    pub fn all() -> &'static [LinterGroup] {
        &[LinterGroup::Header, LinterGroup::Body, LinterGroup::Footer]
    }

    pub fn iter() -> impl Iterator<Item = LinterGroup> {
        LinterGroup::all().iter().copied()
    }
}
