use std::path::{Path, PathBuf};

use commitfmt_cc::footer::SeparatorAlignment;
use serde_derive::{Deserialize, Serialize};
use toml::{map::Map, Table, Value};

use crate::{WorkspaceError, WorkspaceResult};

/// List of known config file names
const KNOWN_PATHS: &[&str] = &[".commitfmt.toml", "commitfmt.toml"];

/// Maximum size of the config file
/// If the file is larger than this, return an error.
const MAX_CONFIG_SIZE: u64 = 1_000_000;

/// Additional footer configuration.
///
/// This is used to add additional footers to the commit message.
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct AdditionalFooterConfig {
    pub key: String,
    pub on_conflict: Option<String>,
    pub value_template: Option<String>,
    pub branch_value_pattern: Option<String>,
    pub separator: Option<char>,
    pub alignment: Option<SeparatorAlignment>,
}

/// Lint configuration.
#[derive(Debug, PartialEq, Deserialize, Clone, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct LintConfiguration {
    pub unsafe_fixes: Option<bool>,
}

/// Commit configuration.
///
/// This is used to configure the commit message.
#[derive(Debug, PartialEq, Deserialize, Clone, Serialize, Default)]
pub(crate) struct CommitConfiguration {
    pub extends: Option<String>,

    pub lint: Option<LintConfiguration>,

    #[serde(alias = "additional-footers")]
    pub additional_footers: Option<Vec<AdditionalFooterConfig>>,
}

/// Commit parameters.
///
/// This is used to store the commit message parameters.
#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) struct CommitParams {
    pub config: CommitConfiguration,
    pub lint_values: Map<String, Value>,
}

impl CommitParams {
    /// Parse a TOML string into a `CommitParams` object
    fn parse_toml(data: &str) -> WorkspaceResult<Self> {
        let config: CommitConfiguration = toml::from_str(data)?;

        let config_values = data.parse::<Table>()?;
        let mut lint_values = Map::new();
        if let Some(lint_table) = config_values.get("lint") {
            lint_values = lint_table.as_table().unwrap().clone();
        }

        Ok(Self { config, lint_values })
    }

    /// Open a single configuration file without extending it
    /// and parse it into a `CommitParams` object
    fn open_single(path: &Path) -> WorkspaceResult<Self> {
        if std::fs::metadata(path)?.len() > MAX_CONFIG_SIZE {
            return Err(WorkspaceError::FileTooLarge);
        }
        let data = std::fs::read_to_string(path)?;
        Self::parse_toml(&data)
    }

    pub(crate) fn find_config_path(dir: &Path) -> WorkspaceResult<PathBuf> {
        for path in KNOWN_PATHS {
            let path = dir.join(path);
            if path.exists() {
                return Ok(path);
            }
        }
        Err(WorkspaceError::ConfigNotFound(dir.to_string_lossy().to_string()))
    }

    /// Open configuration from directory
    /// If the file contains an `extends` field, it will be used to extend the configuration.
    pub(crate) fn open(config_path: &Path) -> WorkspaceResult<CommitParams> {
        if !config_path.is_file() || !config_path.exists() {
            return Err(WorkspaceError::ConfigNotFound(
                config_path.to_string_lossy().to_string(),
            ));
        }

        let target_params = Self::open_single(config_path)?;
        if let Some(extends) = &target_params.config.extends {
            let parent_path = config_path.parent().unwrap().join(extends);
            let mut parent_params = Self::open_single(&parent_path)?;
            parent_params.extend(&target_params);

            Ok(parent_params)
        } else {
            Ok(target_params)
        }
    }
}

impl CommitParams {
    /// Merges the current configuration with the provided configuration.
    ///
    /// This method modifies the current object in place, applying values from `other`.
    /// Values from `other` take precedence and override existing settings:
    ///
    /// - `lint`: completely replaced with configuration from `other`, if present
    /// - `footers`: additional footers from `other` are appended to existing ones
    /// - `params`: parameters from `other` are merged with existing ones
    ///
    /// The `extends` field is ignored and not processed.
    pub(crate) fn extend(&mut self, other: &CommitParams) {
        if let Some(other_lint) = &other.config.lint {
            self.config.lint = Some(other_lint.clone());
        }

        if let Some(other_footers) = &other.config.additional_footers {
            if let Some(self_footers) = &mut self.config.additional_footers {
                for footer in other_footers {
                    self_footers.push(footer.clone());
                }
            } else {
                self.config.additional_footers = Some(other_footers.clone());
            }
        }

        self.lint_values.extend(other.lint_values.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use commitfmt_cc::footer::SeparatorAlignment;

    #[test]
    fn test_parse_toml() {
        let params = CommitParams::parse_toml(
            "
[lint]
unsafe-fixes = true

[lint.header]
full-stop = false

[[additional-footers]]
key = \"Footer\"
on-conflict = \"error\"
value-template = \"{{ echo $USER }}\"
",
        )
        .unwrap();
        assert_eq!(params.config.extends, None);
        assert_eq!(params.config.lint.as_ref().unwrap().unsafe_fixes, Some(true));

        assert!(params.config.additional_footers.is_some());
        let footers = params.config.additional_footers.as_ref().unwrap();
        assert_eq!(footers.len(), 1);
        assert_eq!(footers[0].key, "Footer");
        assert_eq!(footers[0].on_conflict, Some("error".to_string()));
        assert_eq!(footers[0].value_template, Some("{{ echo $USER }}".to_string()));

        let header_table = params.lint_values.get("header").unwrap().as_table().unwrap();
        assert_eq!(header_table.get("full-stop").unwrap(), &Value::Boolean(false));
    }

    #[test]
    fn test_extend_with_lint_config() {
        let mut params = CommitParams {
            config: CommitConfiguration {
                extends: Some("base".to_string()),
                lint: Some(LintConfiguration { unsafe_fixes: Some(false) }),
                additional_footers: None,
            },
            lint_values: Map::new(),
        };

        let other = CommitParams {
            config: CommitConfiguration {
                extends: Some("ignored".to_string()),
                lint: Some(LintConfiguration { unsafe_fixes: Some(true) }),
                additional_footers: None,
            },
            lint_values: Map::new(),
        };

        params.extend(&other);

        assert_eq!(params.config.lint.as_ref().unwrap().unsafe_fixes, Some(true));
        assert_eq!(params.config.extends, Some("base".to_string())); // should not change
    }

    #[test]
    fn test_extend_with_footers() {
        let mut params = CommitParams {
            config: CommitConfiguration {
                extends: None,
                lint: None,
                additional_footers: Some(vec![AdditionalFooterConfig {
                    key: "existing".to_string(),
                    on_conflict: Some("error".to_string()),
                    value_template: Some("existing template".to_string()),
                    branch_value_pattern: None,
                    separator: None,
                    alignment: None,
                }]),
            },
            lint_values: Map::new(),
        };

        let other = CommitParams {
            config: CommitConfiguration {
                extends: None,
                lint: None,
                additional_footers: Some(vec![AdditionalFooterConfig {
                    key: "new".to_string(),
                    on_conflict: Some("skip".to_string()),
                    value_template: Some("new template".to_string()),
                    branch_value_pattern: Some("pattern".to_string()),
                    separator: None,
                    alignment: None,
                }]),
            },
            lint_values: Map::new(),
        };

        params.extend(&other);

        let footers = params.config.additional_footers.as_ref().unwrap();
        assert_eq!(footers.len(), 2);
        assert_eq!(footers[0].key, "existing");
        assert_eq!(footers[1].key, "new");
    }

    #[test]
    fn test_extend_with_empty_footers() {
        let mut params = CommitParams {
            config: CommitConfiguration {
                extends: None,
                lint: None,
                additional_footers: None,
            },
            lint_values: Map::new(),
        };

        let other = CommitParams {
            config: CommitConfiguration {
                extends: None,
                lint: None,
                additional_footers: Some(vec![AdditionalFooterConfig {
                    key: "first".to_string(),
                    on_conflict: None,
                    value_template: Some("template".to_string()),
                    branch_value_pattern: None,
                    separator: None,
                    alignment: None,
                }]),
            },
            lint_values: Map::new(),
        };

        params.extend(&other);

        let footers = params.config.additional_footers.as_ref().unwrap();
        assert_eq!(footers.len(), 1);
        assert_eq!(footers[0].key, "first");
    }

    #[test]
    fn test_extend_with_params() {
        let mut params = CommitParams {
            config: CommitConfiguration::default(),
            lint_values: {
                let mut map = Map::new();
                map.insert(
                    "existing_key".to_string(),
                    Value::String("existing_value".to_string()),
                );
                map
            },
        };

        let other = CommitParams {
            config: CommitConfiguration::default(),
            lint_values: {
                let mut map = Map::new();
                map.insert("new_key".to_string(), Value::String("new_value".to_string()));
                map.insert(
                    "existing_key".to_string(),
                    Value::String("overridden_value".to_string()),
                );
                map
            },
        };

        params.extend(&other);

        assert_eq!(
            params.lint_values.get("new_key").unwrap(),
            &Value::String("new_value".to_string())
        );
        assert_eq!(
            params.lint_values.get("existing_key").unwrap(),
            &Value::String("overridden_value".to_string())
        );
    }

    #[test]
    fn test_extend_with_none_values() {
        let mut params = CommitParams {
            config: CommitConfiguration {
                extends: Some("test".to_string()),
                lint: Some(LintConfiguration { unsafe_fixes: Some(true) }),
                additional_footers: Some(vec![]),
            },
            lint_values: Map::new(),
        };

        let other = CommitParams {
            config: CommitConfiguration {
                extends: None,
                lint: None,
                additional_footers: None,
            },
            lint_values: Map::new(),
        };

        params.extend(&other);

        // Original values should remain unchanged
        assert_eq!(params.config.extends, Some("test".to_string()));
        assert_eq!(params.config.lint.as_ref().unwrap().unsafe_fixes, Some(true));
        assert_eq!(params.config.additional_footers.as_ref().unwrap().len(), 0);
    }

    #[test]
    fn test_extend_complete_scenario() {
        let mut params = CommitParams {
            config: CommitConfiguration {
                extends: Some("base".to_string()),
                lint: Some(LintConfiguration { unsafe_fixes: Some(false) }),
                additional_footers: Some(vec![AdditionalFooterConfig {
                    key: "base_footer".to_string(),
                    on_conflict: Some("error".to_string()),
                    value_template: Some("base template".to_string()),
                    branch_value_pattern: None,
                    separator: None,
                    alignment: None,
                }]),
            },
            lint_values: {
                let mut map = Map::new();
                map.insert("base_param".to_string(), Value::String("base_value".to_string()));
                map
            },
        };

        let other = CommitParams {
            config: CommitConfiguration {
                extends: Some("should_be_ignored".to_string()),
                lint: Some(LintConfiguration { unsafe_fixes: Some(true) }),
                additional_footers: Some(vec![AdditionalFooterConfig {
                    key: "other_footer".to_string(),
                    on_conflict: Some("skip".to_string()),
                    value_template: None,
                    branch_value_pattern: Some("pattern".to_string()),
                    separator: None,
                    alignment: None,
                }]),
            },
            lint_values: {
                let mut map = Map::new();
                map.insert(
                    "other_param".to_string(),
                    Value::String("other_value".to_string()),
                );
                map.insert(
                    "base_param".to_string(),
                    Value::String("overridden_value".to_string()),
                );
                map
            },
        };

        params.extend(&other);

        // Check lint config was replaced
        assert_eq!(params.config.lint.as_ref().unwrap().unsafe_fixes, Some(true));

        // Check footers were appended
        let footers = params.config.additional_footers.as_ref().unwrap();
        assert_eq!(footers.len(), 2);
        assert_eq!(footers[0].key, "base_footer");
        assert_eq!(footers[1].key, "other_footer");

        // Check params were merged with override
        assert_eq!(
            params.lint_values.get("base_param").unwrap(),
            &Value::String("overridden_value".to_string())
        );
        assert_eq!(
            params.lint_values.get("other_param").unwrap(),
            &Value::String("other_value".to_string())
        );

        // Check extends was not modified
        assert_eq!(params.config.extends, Some("base".to_string()));
    }

    #[test]
    fn test_open_single_simple_config() {
        let path = Path::new("resources/testdata/simple.toml");
        let params = CommitParams::open_single(path).unwrap();

        assert_eq!(params.config.extends, None);
        assert_eq!(params.config.lint.as_ref().unwrap().unsafe_fixes, Some(true));

        let footers = params.config.additional_footers.as_ref().unwrap();
        assert_eq!(footers.len(), 1);
        assert_eq!(footers[0].key, "Footer");
        assert_eq!(footers[0].on_conflict, Some("error".to_string()));
        assert_eq!(footers[0].value_template, Some("{{ echo $USER }}".to_string()));

        let header_table = params.lint_values.get("header").unwrap().as_table().unwrap();
        assert_eq!(header_table.get("full-stop").unwrap(), &Value::Boolean(false));
    }

    #[test]
    fn test_open_single_children_config() {
        let path = Path::new("resources/testdata/children.toml");
        let params = CommitParams::open_single(path).unwrap();

        assert_eq!(params.config.extends, Some("simple.toml".to_string()));
        assert_eq!(params.config.lint, None);
        assert_eq!(params.config.additional_footers, None);
        assert!(params.lint_values.is_empty());
    }

    #[test]
    fn test_open_with_extends() {
        let path = Path::new("resources/testdata/children.toml");
        let params = CommitParams::open(path).unwrap();

        // Config should come from parent (simple.toml)
        assert_eq!(params.config.lint.as_ref().unwrap().unsafe_fixes, Some(true));

        let footers = params.config.additional_footers.as_ref().unwrap();
        assert_eq!(footers.len(), 1);
        assert_eq!(footers[0].key, "Footer");
        assert_eq!(footers[0].on_conflict, Some("error".to_string()));

        let header_table = params.lint_values.get("header").unwrap().as_table().unwrap();
        assert_eq!(header_table.get("full-stop").unwrap(), &Value::Boolean(false));
    }

    #[test]
    fn test_open_without_extends() {
        let path = Path::new("resources/testdata/simple.toml");
        let params = CommitParams::open(path).unwrap();

        // Should be same as open_single since no extends
        assert_eq!(params.config.extends, None);
        assert_eq!(params.config.lint.as_ref().unwrap().unsafe_fixes, Some(true));

        let footers = params.config.additional_footers.as_ref().unwrap();
        assert_eq!(footers.len(), 1);
        assert_eq!(footers[0].key, "Footer");
    }

    #[test]
    fn test_open_nonexistent_file() {
        let path = Path::new("nonexistent/path/config.toml");
        let result = CommitParams::open_single(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_open_invalid_toml() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "invalid toml [[[").unwrap();
        temp_file.flush().unwrap();

        let result = CommitParams::open_single(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_toml_with_separator_and_alignment() {
        let params = CommitParams::parse_toml(
            "
[lint]
unsafe-fixes = true

[[additional-footers]]
key = \"Footer\"
on-conflict = \"error\"
value-template = \"{{ echo $USER }}\"
separator = '#'
alignment = \"right\"
",
        )
        .unwrap();

        assert!(params.config.additional_footers.is_some());
        let footers = params.config.additional_footers.as_ref().unwrap();
        assert_eq!(footers.len(), 1);
        assert_eq!(footers[0].key, "Footer");
        assert_eq!(footers[0].separator, Some('#'));
        assert_eq!(footers[0].alignment, Some(SeparatorAlignment::Right));
    }
}
