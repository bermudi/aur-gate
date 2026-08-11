//! TTY-aware stderr reporter.

use std::io::IsTerminal;

use crate::classifier::Reporter;

/// Render untrusted candidate/provider text without terminal control bytes.
/// Raw evidence remains unchanged on disk; only presentation is escaped.
pub fn terminal_safe(text: &str) -> String {
    render_safe(text.as_bytes(), false)
}

/// Escape untrusted bytes while preserving patch line structure for paging.
pub fn document_safe(text: &str) -> String {
    document_safe_bytes(text.as_bytes())
}

pub fn document_safe_bytes(bytes: &[u8]) -> String {
    render_safe(bytes, true)
}

pub fn error(text: &str) {
    eprintln!("error: {}", terminal_safe(text));
}

/// Escape terminal-control bytes and invalid UTF-8 while preserving valid
/// Unicode (aur-gate's own glyphs like `▶`/`→`/`—`, plus legitimate CJK and
/// accented evidence text). Only true control characters — C0 (incl. ESC),
/// DEL, and C1 codes — can drive a terminal, so those are the bytes we escape;
/// their UTF-8 encoding is rendered as `\xNN` so the sequence cannot execute.
fn render_safe(bytes: &[u8], preserve_newlines: bool) -> String {
    let mut safe = String::with_capacity(bytes.len());
    for chunk in bytes.utf8_chunks() {
        for c in chunk.valid().chars() {
            match c {
                '\n' if preserve_newlines => safe.push('\n'),
                '\n' => safe.push_str("\\n"),
                '\r' => safe.push_str("\\r"),
                '\t' => safe.push_str("\\t"),
                c if c.is_control() => {
                    let mut buf = [0u8; 4];
                    for &b in c.encode_utf8(&mut buf).as_bytes() {
                        safe.push_str(&format!("\\x{b:02x}"));
                    }
                }
                _ => safe.push(c),
            }
        }
        // Lone/overlong bytes that fail to decode as UTF-8 are not control
        // codes, but escape them defensively so output stays unambiguous.
        for &byte in chunk.invalid() {
            safe.push_str(&format!("\\x{byte:02x}"));
        }
    }
    safe
}

pub struct StderrReporter {
    color: bool,
}

impl StderrReporter {
    pub fn new() -> Self {
        Self {
            color: std::io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none(),
        }
    }

    fn styled(&self, code: &str, text: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{text}\x1b[0m")
        } else {
            text.to_owned()
        }
    }

    fn hits(&self, label: &str, code: &str, tag: &str, hits: &[String], limit: usize) {
        eprintln!("  {} [{}]", self.styled(code, label), terminal_safe(tag));
        for hit in hits.iter().take(limit) {
            eprintln!("      {}", terminal_safe(hit));
        }
    }
}

impl Default for StderrReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for StderrReporter {
    fn block_hits(&mut self, tag: &str, hits: &[String], limit: usize) {
        self.hits("BLOCK", "1;31", tag, hits, limit);
    }

    fn review_hits(&mut self, tag: &str, hits: &[String], limit: usize) {
        self.hits("review", "1;33", tag, hits, limit);
    }

    fn review_msg(&mut self, msg: &str) {
        eprintln!("  {} {}", self.styled("1;33", "review"), terminal_safe(msg));
    }

    fn dim(&mut self, msg: &str) {
        eprintln!("{}", self.styled("2", &terminal_safe(msg)));
    }

    fn review_needed(&mut self, summary: &str, detail: &str) {
        eprintln!(
            "  {} {}",
            self.styled("1;33", "review needed —"),
            terminal_safe(summary)
        );
        eprintln!("      added: {}", terminal_safe(detail));
    }
}

#[cfg(test)]
mod tests {
    use super::{document_safe, document_safe_bytes, terminal_safe};

    #[test]
    fn terminal_safe_escapes_controls_but_preserves_unicode() {
        // C0 controls (ESC, CR, LF, tab) and DEL are escaped — this is the
        // terminal-injection defense.
        assert_eq!(
            terminal_safe("ok\x1b[2J\r\n\t\u{007f}"),
            "ok\\x1b[2J\\r\\n\\t\\x7f"
        );
        // C1 8-bit control codes are escaped as their UTF-8 bytes.
        assert_eq!(terminal_safe("\u{0080}"), "\\xc2\\x80");
        // Valid Unicode passes through, including aur-gate's own UI glyphs and
        // legitimate CJK/accented evidence text.
        assert_eq!(
            terminal_safe("▶ cursor-bin — proceed"),
            "▶ cursor-bin — proceed"
        );
        assert_eq!(terminal_safe("→ ✓ ✗ ⚠ é 漢"), "→ ✓ ✗ ⚠ é 漢");
        // Document mode preserves newlines for paging while still escaping ESC;
        // the byte path also escapes non-UTF-8 bytes (terminal_safe takes &str,
        // which cannot hold invalid UTF-8 by construction).
        assert_eq!(document_safe("a\n\x1bb"), "a\n\\x1bb");
        assert_eq!(document_safe_bytes(b"a\n\xffb"), "a\n\\xffb");
    }
}
