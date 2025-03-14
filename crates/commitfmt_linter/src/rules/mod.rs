pub mod linter_group;
mod names;

pub mod body;
pub mod header;
pub mod footer;

pub use linter_group::LinterGroup;
pub use names::Rule;

#[derive(Default, Debug, PartialEq)]
pub struct Settings {
    pub body: body::Settings,
    pub header: header::Settings,
    pub footer: footer::Settings,
}
