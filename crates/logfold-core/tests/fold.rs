use logfold_core::Folder;

#[test]
fn folds_and_sorts_by_count() {
    let mut folder = Folder::new();

    for line in ["user 1 in", "user 2 in", "timeout 30", "user 3 in"] {
        folder.add(line);
    }

    let patterns = folder.finish();

    assert_eq!(patterns.len(), 2);
    assert_eq!(patterns[0].text, "user <NUM> in");
    assert_eq!(patterns[0].count, 3);
    assert_eq!(patterns[1].text, "timeout <NUM>");
    assert_eq!(patterns[1].count, 1);
}

#[test]
fn ignores_blank_lines_and_normalizes_spacing() {
    let mut folder = Folder::new();
    for line in ["", "  ", "\t\r\n", " user 1 in ", "user\t2  in"] {
        folder.add(line);
    }
    assert_eq!(
        folder.finish(),
        vec![logfold_core::Pattern {
            text: "user <NUM> in".into(),
            count: 2
        }]
    );
}

#[test]
fn sorts_equal_counts_lexicographically() {
    for lines in [["zebra", "alpha", "middle"], ["middle", "zebra", "alpha"]] {
        let mut folder = Folder::new();
        for line in lines {
            folder.add(line);
        }
        let patterns = folder.finish();
        let names: Vec<_> = patterns
            .iter()
            .map(|pattern| pattern.text.as_str())
            .collect();
        assert_eq!(names, ["alpha", "middle", "zebra"]);
    }
}

#[test]
fn folds_realistic_logs_without_merging_different_levels() {
    let mut folder = Folder::new();
    for line in [
        "2026-10-15T12:30:01Z INFO request 123e4567-e89b-12d3-a456-426614174000 from 10.0.0.1 took 12ms",
        "2026-10-15T12:30:02.123Z INFO request 223e4567-e89b-12d3-a456-426614174001 from 192.168.1.5 took 1.5ms",
        "2026-10-15T12:30:03Z ERROR request 323e4567-e89b-12d3-a456-426614174002 from 127.0.0.1 took 5ms",
    ] {
        folder.add(line);
    }
    let patterns = folder.finish();
    assert_eq!(patterns.len(), 2);
    assert_eq!(patterns[0].count, 2);
    assert_eq!(
        patterns[0].text,
        "<TIMESTAMP> INFO request <UUID> from <IP> took <NUM>ms"
    );
    assert_eq!(patterns[1].count, 1);
    assert!(patterns[1].text.contains(" ERROR "));
}

#[test]
fn keeps_different_word_identifiers_separate() {
    let mut folder = Folder::new();
    folder.add("api-v1 ready");
    folder.add("api-v2 ready");
    assert_eq!(folder.finish().len(), 2);
}
