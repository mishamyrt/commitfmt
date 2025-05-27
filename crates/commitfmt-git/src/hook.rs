#[derive(Debug, Clone, Copy)]
pub enum HookType {
    PrepareCommitMsg,
    ApplyPatchMsg,
}

impl HookType {
    pub fn as_str(&self) -> &str {
        match self {
            HookType::PrepareCommitMsg => "prepare-commit-msg",
            HookType::ApplyPatchMsg => "applypatch-msg",
        }
    }
}
