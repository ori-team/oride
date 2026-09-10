//! Clipboard: arboard (sistema) + OSC52 (SSH) + fallback interno.

use std::sync::Mutex;

#[cfg(not(test))]
use std::io::{self, Write};

static INTERNAL_BUFFER: Mutex<String> = Mutex::new(String::new());
static SYSTEM_CLIPBOARD: Mutex<Option<arboard::Clipboard>> = Mutex::new(None);

fn with_system_clipboard<T>(
    operation: impl FnOnce(&mut arboard::Clipboard) -> Result<T, String>,
) -> Result<T, String> {
    let mut clipboard_guard = SYSTEM_CLIPBOARD
        .lock()
        .map_err(|poison_error| format!("clipboard lock poisoned: {poison_error}"))?;

    if clipboard_guard.is_none() {
        if let Ok(clipboard) = arboard::Clipboard::new() {
            *clipboard_guard = Some(clipboard);
        }
    }

    if let Some(clipboard) = clipboard_guard.as_mut() {
        operation(clipboard)
    } else {
        Err("clipboard indisponível".to_string())
    }
}

/// Copia texto para o clipboard do sistema; se falhar, usa buffer interno + OSC52.
pub fn copy_text(text: &str) -> Result<(), String> {
    if let Ok(mut buffer_guard) = INTERNAL_BUFFER.lock() {
        *buffer_guard = text.to_string();
    }
    // OSC52: funciona em muitos terminais via SSH.
    #[cfg(not(test))]
    let _ = write_osc52(text);

    let _ = with_system_clipboard(|clipboard| {
        clipboard
            .set_text(text.to_string())
            .map_err(|error| format!("clipboard: {error}"))
    });

    Ok(())
}

/// Lê clipboard do sistema; se vazio/falhar, usa buffer interno.
#[must_use]
pub fn paste_text() -> String {
    let system_result = with_system_clipboard(|clipboard| {
        clipboard
            .get_text()
            .map_err(|error| format!("clipboard: {error}"))
    });

    if let Ok(content) = system_result {
        if !content.is_empty() {
            return content;
        }
    }

    internal_text()
}

/// Só o buffer interno (testes / fallback sem X11).
#[must_use]
pub fn internal_text() -> String {
    INTERNAL_BUFFER
        .lock()
        .map(|guard| guard.clone())
        .unwrap_or_default()
}

#[cfg(not(test))]
fn write_osc52(text: &str) -> io::Result<()> {
    use base64::Engine;
    let base64_encoded = base64::engine::general_purpose::STANDARD.encode(text.as_bytes());
    // OSC 52: ESC ] 52 ; c ; <base64> BEL
    let escape_sequence = format!("\x1b]52;c;{base64_encoded}\x07");
    let mut standard_output = io::stdout();
    standard_output.write_all(escape_sequence.as_bytes())?;
    standard_output.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_and_paste_internal_fallback() {
        let sample = "Hello Oride Clipboard";
        assert!(copy_text(sample).is_ok());
        assert_eq!(internal_text(), sample);
    }
}
