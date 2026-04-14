pub mod claude;

use anyhow::Result;

use crate::core::Session;

/// Provider abstraction so more local session formats can be added later.
pub trait Provider {
    fn load_session(session_id: &str) -> Result<Session>;
}
