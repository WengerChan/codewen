use std::path::PathBuf;

use codewen_utils_absolute_path::AbsolutePathBuf;

/// Runtime paths needed by exec-server child processes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecServerRuntimePaths {
    /// Stable path to the Codex executable used to launch hidden helper modes.
    pub codewen_self_exe: AbsolutePathBuf,
    /// Path to the Linux sandbox helper alias used when the platform sandbox
    /// needs to re-enter Codex by argv0.
    pub codewen_linux_sandbox_exe: Option<AbsolutePathBuf>,
}

impl ExecServerRuntimePaths {
    pub fn from_optional_paths(
        codewen_self_exe: Option<PathBuf>,
        codewen_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        let codewen_self_exe = codewen_self_exe.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Codex executable path is not configured",
            )
        })?;
        Self::new(codewen_self_exe, codewen_linux_sandbox_exe)
    }

    pub fn new(
        codewen_self_exe: PathBuf,
        codewen_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            codewen_self_exe: absolute_path(codewen_self_exe)?,
            codewen_linux_sandbox_exe: codewen_linux_sandbox_exe.map(absolute_path).transpose()?,
        })
    }
}

fn absolute_path(path: PathBuf) -> std::io::Result<AbsolutePathBuf> {
    AbsolutePathBuf::from_absolute_path(path.as_path())
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
}
