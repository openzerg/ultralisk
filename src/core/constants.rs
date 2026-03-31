pub const HIDDEN_SESSION_ID: &str = "00000000-0000-0000-0000-000000000000";

pub fn is_hidden_session(session_id: &str) -> bool {
    session_id == HIDDEN_SESSION_ID
}
