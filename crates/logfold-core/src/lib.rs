use regex::Regex;

pub fn mask_line(line: &str) -> String {
    let ip = Regex::new(r"\b\d{1,3}(\.\d{1,3}){3}\b").unwrap();
    let num = Regex::new(r"\d+").unwrap();

    let line = ip.replace_all(line, "<IP>");
    let line = num.replace_all(&line, "<NUM>");
    line.to_string()
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
}