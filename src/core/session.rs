use crate::core::{Message, Role};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub id: String,
    pub messages: Vec<Message>,
}

impl Session {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            messages: Vec::new(),
        }
    }

    /// Normalize the conversation as we build it:
    /// - ignore empty messages
    /// - merge consecutive messages from the same role, because Claude stores
    ///   streaming/tool-interleaved assistant output as multiple JSONL events.
    pub fn push_message(&mut self, role: Role, content: impl Into<String>) {
        let content = content.into();
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return;
        }

        if let Some(previous) = self.messages.last_mut()
            && previous.role == role
        {
            previous.content.push_str("\n\n");
            previous.content.push_str(trimmed);
            return;
        }

        self.messages.push(Message {
            role,
            content: trimmed.to_owned(),
        });
    }
}
