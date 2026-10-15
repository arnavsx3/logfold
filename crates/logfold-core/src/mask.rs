use std::sync::LazyLock;

use regex::Regex;

static UUID_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b",
    )
    .unwrap()
});

static IP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b\d{1,3}(\.\d{1,3}){3}\b").unwrap());

static NUM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

/// Replaces the changing parts of a log line with placeholders.
/// Order matters: UUIDs first, then IPs, then plain numbers.
pub fn mask_line(line: &str) -> String {
    let line = UUID_RE.replace_all(line, "<UUID>");
    let line = IP_RE.replace_all(&line, "<IP>");
    let line = NUM_RE.replace_all(&line, "<NUM>");
    line.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_ip_and_numbers() {
        assert_eq!(
            mask_line("user 123 logged in from 10.0.0.5"),
            "user <NUM> logged in from <IP>"
        );
    }

    #[test]
    fn masks_uuid() {
        assert_eq!(
            mask_line("req 123e4567-e89b-12d3-a456-426614174000 done"),
            "req <UUID> done"
        );
    }

    #[test]
    fn leaves_plain_text_alone() {
        assert_eq!(mask_line("server started"), "server started");
    }
}