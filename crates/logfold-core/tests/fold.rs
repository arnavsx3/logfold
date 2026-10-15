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
