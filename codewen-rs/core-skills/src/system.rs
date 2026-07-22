pub(crate) use codewen_skills::install_system_skills;
pub(crate) use codewen_skills::system_cache_root_dir;

use codewen_utils_absolute_path::AbsolutePathBuf;

pub(crate) fn uninstall_system_skills(codewen_home: &AbsolutePathBuf) {
    let _ = std::fs::remove_dir_all(system_cache_root_dir(codewen_home));
}
