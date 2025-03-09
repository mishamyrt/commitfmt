use serde_derive::Deserialize;
use std::cell::RefCell;

use crate::settings::{AdditionalFooter, FormattingSettings};

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct FormattingSettingsConfig {
    #[serde(alias = "unsafe-fixes")]
    pub unsafe_fixes: Option<bool>,

    pub footers: Option<Vec<AdditionalFooter>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct FormattingConfig {
    pub formatting: Option<FormattingSettingsConfig>,
}

impl FormattingConfig {
    pub(crate) fn to_settings(&self) -> FormattingSettings {
        let Some(formatting) = self.formatting.as_ref() else {
            return FormattingSettings {
                unsafe_fixes: false,
                footers: RefCell::new(vec![]),
            };
        };
        let footers = RefCell::new(formatting.footers.clone().unwrap_or_default());
        FormattingSettings {
            unsafe_fixes: formatting.unsafe_fixes.unwrap_or(false),
            footers,
        }
    }
}
