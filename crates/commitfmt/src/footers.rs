use commitfmt_cc::{Footer, SeparatorAlignment};
use commitfmt_config::{settings::AdditionalFooterKind, AdditionalFooter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FooterError {
    #[error("Footer template cannot be rendered: {0}")]
    TemplateError(#[from] commitfmt_tpl_exec::TplError),
}

pub(crate) struct FooterAdder<'f>(pub &'f mut Vec<Footer>);

impl<'f> FooterAdder<'f> {
    pub(crate) fn append(&mut self, footer: &AdditionalFooter) -> Result<(), FooterError> {
        let value = match footer.kind {
            AdditionalFooterKind::Template => {
                match commitfmt_tpl_exec::render(footer.value_template.as_ref().unwrap()) {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(FooterError::TemplateError(err));
                    }
                }
            }
            AdditionalFooterKind::BranchPattern => {
                unreachable!();
            }
        };
        self.0.push(Footer {
            key: footer.key.clone(),
            separator: Footer::DEFAULT_SEPARATOR.chars().next().unwrap(),
            alignment: SeparatorAlignment::default(),
            value,
        });

        Ok(())
    }
}
