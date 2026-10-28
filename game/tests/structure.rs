//! One file, one job: every source file stays within `LIMIT` lines of code. Blank lines, comments
//! and the `#[cfg(test)]` module do not count; test-only files (`tests.rs`) are left out.

use std::fs;
use std::path::Path;

const LIMIT: usize = 200;

/// Lines of code above the file's tests.
fn code_lines(text: &str) -> usize {
    text.lines()
        .take_while(|line| line.trim() != "#[cfg(test)]")
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with("//"))
        .count()
}

fn too_long(dir: &Path, found: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            too_long(&path, found);
        } else if path.extension().is_some_and(|e| e == "rs") && !path.ends_with("tests.rs") {
            let lines = code_lines(&fs::read_to_string(&path).unwrap());
            if lines > LIMIT {
                found.push(format!("{} has {lines} lines of code", path.display()));
            }
        }
    }
}

#[test]
fn files_stay_within_the_limit() {
    let mut found = Vec::new();
    too_long(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut found);
    assert!(found.is_empty(), "over {LIMIT} lines of code, split by job:\n{}", found.join("\n"));
}

#[test]
fn comments_and_tests_do_not_count() {
    let text = "//! Doc.\n\nfn a() {}\n    // note\nfn b() {}\n#[cfg(test)]\nmod tests {}\n";
    assert_eq!(code_lines(text), 2);
}
