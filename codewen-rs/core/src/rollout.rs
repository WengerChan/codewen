use crate::config::Config;
pub use codewen_rollout::ARCHIVED_SESSIONS_SUBDIR;
pub use codewen_rollout::Cursor;
pub use codewen_rollout::INTERACTIVE_SESSION_SOURCES;
pub use codewen_rollout::RolloutRecorder;
pub use codewen_rollout::RolloutRecorderParams;
pub use codewen_rollout::SESSIONS_SUBDIR;
pub use codewen_rollout::SessionMeta;
pub use codewen_rollout::SortDirection;
pub use codewen_rollout::ThreadItem;
pub use codewen_rollout::ThreadSortKey;
pub use codewen_rollout::ThreadsPage;
pub use codewen_rollout::append_thread_name;
pub use codewen_rollout::find_archived_thread_path_by_id_str;
#[deprecated(note = "use find_thread_path_by_id_str")]
pub use codewen_rollout::find_conversation_path_by_id_str;
pub use codewen_rollout::find_thread_meta_by_name_str;
pub use codewen_rollout::find_thread_name_by_id;
pub use codewen_rollout::find_thread_names_by_ids;
pub use codewen_rollout::find_thread_path_by_id_str;
pub use codewen_rollout::parse_cursor;
pub use codewen_rollout::read_head_for_summary;
pub use codewen_rollout::read_session_meta_line;
pub use codewen_rollout::rollout_date_parts;

impl codewen_rollout::RolloutConfigView for Config {
    fn codewen_home(&self) -> &std::path::Path {
        self.codewen_home.as_path()
    }

    fn sqlite_home(&self) -> &std::path::Path {
        self.sqlite_home.as_path()
    }

    fn cwd(&self) -> &std::path::Path {
        self.cwd.as_path()
    }

    fn model_provider_id(&self) -> &str {
        self.model_provider_id.as_str()
    }

    fn generate_memories(&self) -> bool {
        self.memories.generate_memories
    }
}

pub(crate) mod list {
    pub use codewen_rollout::find_thread_path_by_id_str;
}

#[cfg(test)]
pub(crate) mod recorder {
    pub use codewen_rollout::RolloutRecorder;
}

pub(crate) use crate::session_rollout_init_error::map_session_init_error;

pub(crate) mod truncation {
    pub(crate) use crate::thread_rollout_truncation::*;
}
