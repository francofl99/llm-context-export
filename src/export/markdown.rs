use crate::core::Session;

pub fn render(session: &Session, provider_name: &str) -> String {
    let session_markdown = render_session_markdown(session, provider_name);
    let mut output = String::new();

    // Wrap the raw export with handoff instructions so the pasted content is
    // self-describing when used as context in another LLM.
    output.push_str(&format!(
        "Esta es una sesión exportada de `{provider_name}`:\n\n"
    ));
    output.push_str(&session_markdown);
    output.push_str(
        "\n\nNecesito que la revises, me digas si entendés el contexto, para poder continuar trabajando desde acá.\n",
    );

    output
}

fn render_session_markdown(session: &Session, provider_name: &str) -> String {
    let mut output = String::new();

    output.push_str(&format!("# {provider_name} Session Export\n\n"));
    output.push_str(&format!("Session ID: {}\n\n", session.id));
    output.push_str("## Conversation\n\n");

    for (index, message) in session.messages.iter().enumerate() {
        output.push_str(&format!("### {}\n", message.role));
        output.push_str(&message.content);
        output.push('\n');

        if index + 1 != session.messages.len() {
            output.push_str("\n---\n\n");
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::core::{Role, Session};

    use super::render;

    #[test]
    fn wraps_session_with_handoff_context() {
        let mut session = Session::new("session-123");
        session.push_message(Role::User, "Hola");
        session.push_message(Role::Assistant, "Entendido");

        let markdown = render(&session, "Claude");

        assert!(markdown.contains("Esta es una sesión exportada de `Claude`:"));
        assert!(markdown.contains("# Claude Session Export"));
        assert!(markdown.contains("Session ID: session-123"));
        assert!(markdown.contains("Necesito que la revises"));
    }
}
