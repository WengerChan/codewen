use std::io;

use codewen_extension_api::LoadUserInstructionsFuture;
use codewen_extension_api::LoadedUserInstructions;
use codewen_extension_api::UserInstructions;
use codewen_extension_api::UserInstructionsProvider;
use codewen_utils_absolute_path::AbsolutePathBuf;

const DEFAULT_AGENTS_MD_FILENAME: &str = "AGENTS.md";
const LOCAL_AGENTS_MD_FILENAME: &str = "AGENTS.override.md";

/// Loads user instructions from a Codex home directory.
#[derive(Clone, Debug)]
pub struct CodewenHomeUserInstructionsProvider {
    codewen_home: AbsolutePathBuf,
}

impl CodewenHomeUserInstructionsProvider {
    /// Creates a provider rooted at the supplied absolute Codex home directory.
    pub fn new(codewen_home: AbsolutePathBuf) -> Self {
        Self { codewen_home }
    }

    async fn load_from_codewen_home(&self) -> LoadedUserInstructions {
        let mut warnings = Vec::new();
        for candidate in [LOCAL_AGENTS_MD_FILENAME, DEFAULT_AGENTS_MD_FILENAME] {
            let path = self.codewen_home.join(candidate);
            match tokio::fs::metadata(path.as_path()).await {
                Ok(metadata) if !metadata.is_file() => continue,
                Ok(_) => {}
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => {
                    warnings.push(format!(
                        "Failed to read global AGENTS.md instructions from `{}`: {err}",
                        path.display()
                    ));
                    continue;
                }
            }
            let data = match tokio::fs::read(path.as_path()).await {
                Ok(data) => data,
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => {
                    warnings.push(format!(
                        "Failed to read global AGENTS.md instructions from `{}`: {err}",
                        path.display()
                    ));
                    continue;
                }
            };
            let contents = String::from_utf8_lossy(&data);
            let trimmed = contents.trim();
            if !trimmed.is_empty() {
                return LoadedUserInstructions {
                    instructions: Some(UserInstructions {
                        text: trimmed.to_string(),
                        source: path,
                    }),
                    warnings,
                };
            }
        }
        LoadedUserInstructions {
            instructions: None,
            warnings,
        }
    }
}

impl UserInstructionsProvider for CodewenHomeUserInstructionsProvider {
    fn load_user_instructions(&self) -> LoadUserInstructionsFuture<'_> {
        Box::pin(self.load_from_codewen_home())
    }
}

#[cfg(test)]
mod tests;
