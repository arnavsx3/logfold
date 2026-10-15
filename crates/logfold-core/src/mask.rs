use std::{net::Ipv4Addr, sync::LazyLock};

use regex::{Captures, Regex};

static TIMESTAMP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b[0-9]{4}-[0-9]{2}-[0-9]{2}[T ][0-9]{2}:[0-9]{2}:[0-9]{2}(?:[.,][0-9]+)?(?:[zZ]|[+-][0-9]{2}:?[0-9]{2})?\b",
    )
    .expect("valid timestamp regex")
});

static UUID_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b")
        .expect("valid UUID regex")
});

static IP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b[0-9]{1,3}(?:\.[0-9]{1,3}){3}\b").expect("valid IPv4 regex"));

static HEX_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b0[xX][0-9a-fA-F]+\b").expect("valid hex regex"));

static NUM_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?P<prefix>^|[^\w])[-+]?[0-9]+(?:\.[0-9]+)?(?:[eE][-+]?[0-9]+)?")
        .expect("valid number regex")
});

/// Normalizes whitespace and replaces variable values with placeholders.
///
/// Recognizes ISO-style timestamps, UUIDs, IPv4 addresses, `0x` hex values,
/// and numbers (including decimals, exponents, and values such as `30ms`).
/// Digits inside words such as `worker2` and `v1` are preserved.
/// More specific values are masked before numbers to avoid fragmenting them.
pub fn mask_line(line: &str) -> String {
    let mut normalized = String::with_capacity(line.len());
    for word in line.split_whitespace() {
        if !normalized.is_empty() {
            normalized.push(' ');
        }
        normalized.push_str(word);
    }
    if normalized.is_empty() {
        return normalized;
    }

    let line = TIMESTAMP_RE.replace_all(&normalized, "<TIMESTAMP>");
    let line = UUID_RE.replace_all(&line, "<UUID>");
    let line = IP_RE.replace_all(&line, |captures: &Captures<'_>| {
        let address = &captures[0];
        if address.parse::<Ipv4Addr>().is_ok() {
            "<IP>".to_owned()
        } else {
            address.to_owned()
        }
    });
    let line = HEX_RE.replace_all(&line, "<HEX>");
    let line = NUM_RE.replace_all(&line, "${prefix}<NUM>");
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

    #[test]
    fn normalizes_whitespace() {
        assert_eq!(
            mask_line("  user\t42   logged in\r\n"),
            "user <NUM> logged in"
        );
        assert_eq!(mask_line(" \t\r\n"), "");
    }

    #[test]
    fn masks_timestamp_precision_and_timezones() {
        for timestamp in [
            "2026-10-15T12:30:01Z",
            "2026-10-15T12:30:01.123456Z",
            "2026-10-15T18:00:01+05:30",
            "2026-10-15T18:00:01.123+0530",
            "2026-10-15 12:30:01",
            "2026-10-15 12:30:01,123",
        ] {
            assert_eq!(
                mask_line(&format!("[{timestamp}] INFO ready")),
                "[<TIMESTAMP>] INFO ready"
            );
        }
    }

    #[test]
    fn masks_whole_numeric_values() {
        assert_eq!(
            mask_line("duration=12ms delta=-1.5 temp=+23.5 load=2.4e-3 address=0xDEADBEEF"),
            "duration=<NUM>ms delta=<NUM> temp=<NUM> load=<NUM> address=<HEX>"
        );
        assert_eq!(
            mask_line("1,2,3 +4 -5 6.7 8e9"),
            "<NUM>,<NUM>,<NUM> <NUM> <NUM> <NUM> <NUM>"
        );
    }

    #[test]
    fn preserves_digits_inside_words() {
        assert_eq!(
            mask_line("worker2 api-v1 node-42 café3"),
            "worker2 api-v1 node-<NUM> café3"
        );
    }

    #[test]
    fn validates_ipv4_addresses() {
        assert_eq!(mask_line("peer 255.255.255.255:8080"), "peer <IP>:<NUM>");
        assert!(!mask_line("peer 999.999.999.999").contains("<IP>"));
    }

    #[test]
    fn masks_uppercase_uuid() {
        assert_eq!(
            mask_line("req 123E4567-E89B-12D3-A456-426614174000"),
            "req <UUID>"
        );
    }

    #[test]
    fn masking_is_idempotent() {
        let line = "2026-10-15T12:30:01Z user 42 from 10.0.0.1 latency=1.2ms ptr=0xABCD";
        let masked = mask_line(line);
        assert_eq!(mask_line(&masked), masked);
    }
}
