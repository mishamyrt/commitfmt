# commitfmt_format

Library for formatting commit messages. Extends messages with additional footers based on specified configuration.

## Features

- Adding footers to commit messages
- Extracting data from branch names using regular expressions
- Template support with external command execution (via `{{command}}`)
- Flexible conflict handling: skip, append, or error

## Usage

```rust
use commitfmt_cc::Message;
use commitfmt_workspace::AdditionalFooter;
use commitfmt_format::{append_footers, FooterError};

// Create a commit message
let mut message = Message::parse("feat: new feature").unwrap();

// Prepare footers to add
let footers = vec![
    AdditionalFooter {
        key: "Signed-off-by".to_string(),
        value_template: Some("{{git config user.name}} <{{git config user.email}}>".to_string()),
        branch_value_pattern: None,
        on_conflict: commitfmt_workspace::params::OnConflictAction::Append,
    },
    AdditionalFooter {
        key: "Task-ID".to_string(), 
        value_template: None,
        branch_value_pattern: Some("feature/([A-Z]+-[0-9]+)".to_string()),
        on_conflict: commitfmt_workspace::params::OnConflictAction::Append,
    }
];

// Add footers to the message
append_footers(&mut message, &footers, "feature/ABC-123")?;

// Get the formatted message
println!("{}", message);
// Output:
// feat: new feature
//
// Signed-off-by: User Name <user@example.com>
// Task-ID: ABC-123
```
