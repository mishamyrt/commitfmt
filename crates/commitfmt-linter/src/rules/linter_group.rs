#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LinterGroup {
    Header,
    Body,
    Footer,
}

const GROUP_HEADER: &str = "header";
const GROUP_BODY: &str = "body";
const GROUP_FOOTER: &str = "footer";

#[derive(Debug, PartialEq, Eq)]
pub struct LinterParseError;

impl LinterGroup {
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            GROUP_HEADER => LinterGroup::Header,
            GROUP_BODY => LinterGroup::Body,
            GROUP_FOOTER => LinterGroup::Footer,
            _ => return None,
        })
    }

    pub fn as_display(&self) -> &'static str {
        match self {
            LinterGroup::Header => GROUP_HEADER,
            LinterGroup::Body => GROUP_BODY,
            LinterGroup::Footer => GROUP_FOOTER,
        }
    }

    pub fn all() -> &'static [LinterGroup] {
        &[LinterGroup::Header, LinterGroup::Body, LinterGroup::Footer]
    }

    pub fn iter() -> impl Iterator<Item = LinterGroup> {
        LinterGroup::all().iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_name() {
        assert_eq!(LinterGroup::from_name("header"), Some(LinterGroup::Header));
        assert_eq!(LinterGroup::from_name("body"), Some(LinterGroup::Body));
        assert_eq!(LinterGroup::from_name("footer"), Some(LinterGroup::Footer));
        assert_eq!(LinterGroup::from_name("unknown"), None);
    }
}
