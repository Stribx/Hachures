//! The game's words, one file per language: `assets/data/i18n/<code>.ron` maps keys (`menu.play`,
//! `note.1-1.…`) to text. The code and the data files only hold keys. French (`fr.ron`) is the
//! reference: every key is there, and every other language has the same keys. The tests list what a
//! language misses or no longer needs.
//!
//! - `t("menu.play")`: a line.
//! - `tf("hud.wave_stamp", &[&n])`: a line with values; `{}` takes the next one, `{1}` the second,
//!   so a translation can reorder them.
//! - `tp("title.progress", n, &[..])`: `title.progress.one` or `.other` (or `.few`, `.many`…), as
//!   the language's rule says of `n`.
//! - `n_("tool.barricade")`: a key kept in a table, shown later with `t`.

use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::sync::LazyLock;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A language: its code (kept in the settings), its file, and the plural category a count falls in.
struct Language {
    code: &'static str,
    lines: &'static str,
    plural: fn(u64) -> &'static str,
}

/// Every language, French first. Adding one: copy `fr.ron` to `<code>.ron`, translate it, and add a
/// line here with its plural rule (CLDR categories). Its letters must be in IM Fell and Atkinson
/// Hyperlegible (Western European Latin is).
const LANGUAGES: &[Language] = &[
    Language {
        code: "fr",
        lines: include_str!("../assets/data/i18n/fr.ron"),
        plural: |n| if n <= 1 { "one" } else { "other" },
    },
    Language {
        code: "en",
        lines: include_str!("../assets/data/i18n/en.ron"),
        plural: |n| if n == 1 { "one" } else { "other" },
    },
];

/// Each language's lines, by key.
static TABLES: LazyLock<Vec<HashMap<String, String>>> = LazyLock::new(|| {
    LANGUAGES
        .iter()
        .map(|l| {
            ron::from_str(l.lines)
                .unwrap_or_else(|err| panic!("invalid i18n/{}.ron: {err}", l.code))
        })
        .collect()
});

/// The language shown, as an index in `LANGUAGES`, kept here so any code can translate without the
/// save at hand; the settings set it.
static CURRENT: AtomicUsize = AtomicUsize::new(0);

fn index(code: &str) -> usize {
    LANGUAGES.iter().position(|l| l.code == code).unwrap_or(0)
}

fn current() -> usize {
    CURRENT.load(Ordering::Relaxed)
}

/// Show the game in the language `code`; an unknown code is French.
pub fn set(code: &str) {
    CURRENT.store(index(code), Ordering::Relaxed);
}

/// The code of the language shown.
pub fn code() -> &'static str {
    LANGUAGES[current()].code
}

/// The language `code` as it names itself.
pub fn name(code: &str) -> &'static str {
    TABLES[index(code)].get("language.name").map_or(LANGUAGES[index(code)].code, String::as_str)
}

/// The language `dir` (1 or -1) places after `code` in `LANGUAGES`, round the list.
pub fn next(code: &str, dir: i32) -> &'static str {
    LANGUAGES[(index(code) as i32 + dir).rem_euclid(LANGUAGES.len() as i32) as usize].code
}

/// The line `key` in the language shown.
pub fn t(key: &str) -> &str {
    lookup(key, current())
}

/// The line `key` with its slots filled from `args`.
pub fn tf(key: &str, args: &[&dyn Display]) -> String {
    fill(t(key), args)
}

/// `x` as a number to read: one decimal at most, none when whole, the decimal mark of the language
/// shown.
pub fn number(x: f32) -> String {
    if !x.is_finite() {
        return t("common.unreachable").to_string();
    }
    let text = format!("{x:.1}");
    text.strip_suffix(".0").map_or_else(|| text.replace('.', t("language.decimal")), String::from)
}

/// The form of `key` the language shown uses for the count `n` (`key.one`, `key.other`…), filled
/// from `args`.
pub fn tp(key: &str, n: impl Into<u64>, args: &[&dyn Display]) -> String {
    let lang = current();
    let form = format!("{key}.{}", (LANGUAGES[lang].plural)(n.into()));
    let form = if TABLES[lang].contains_key(&form) { form } else { format!("{key}.other") };
    fill(lookup(&form, lang), args)
}

/// Marks a key kept in a table and shown later with `t`, so the tests find it.
pub const fn n_(key: &'static str) -> &'static str {
    key
}

/// `key` in `lang`, else in French, else the key itself (and said once in development).
fn lookup(key: &str, lang: usize) -> &str {
    if let Some(line) = TABLES[lang].get(key) {
        return line;
    }
    missing(key, lang);
    TABLES[0].get(key).map_or(key, String::as_str)
}

/// A key shown without its line, logged once in development: the tests cannot see keys made at run
/// time.
fn missing(key: &str, lang: usize) {
    #[cfg(debug_assertions)]
    {
        static SEEN: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
        let mut seen = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        if !seen.iter().any(|s| s == key) {
            seen.push(key.to_string());
            bevy::log::warn!(
                "no line for {key:?} in assets/data/i18n/{}.ron",
                LANGUAGES[lang].code
            );
        }
    }
    #[cfg(not(debug_assertions))]
    let _ = (key, lang);
}

/// The key under `prefix` whose French line is `text`: for saves that kept the French text itself.
pub fn french_key(prefix: &str, text: &str) -> Option<&'static str> {
    TABLES[0]
        .iter()
        .find(|(k, line)| k.starts_with(prefix) && *line == text)
        .map(|(k, _)| k.as_str())
}

/// `template` with its slots filled from `args`: `{}` takes the next one, `{1}` the second.
pub fn fill(template: &str, args: &[&dyn Display]) -> String {
    let mut out = String::new();
    let mut next = 0;
    let mut rest = template;
    while let Some((before, slot, after)) = split_slot(rest) {
        out.push_str(before);
        let k = slot.unwrap_or_else(|| {
            next += 1;
            next - 1
        });
        if let Some(arg) = args.get(k) {
            let _ = write!(out, "{arg}");
        }
        rest = after;
    }
    out.push_str(rest);
    out
}

/// The text before the first slot, the slot (`None` for `{}`, `Some(k)` for `{k}`), and the text
/// after. Braces holding anything else are not slots and stay as written.
fn split_slot(text: &str) -> Option<(&str, Option<usize>, &str)> {
    let mut from = 0;
    loop {
        let open = from + text[from..].find('{')?;
        let close = open + text[open..].find('}')?;
        let inside = &text[open + 1..close];
        if inside.is_empty() {
            return Some((&text[..open], None, &text[close + 1..]));
        }
        if let Ok(k) = inside.parse::<usize>() {
            return Some((&text[..open], Some(k), &text[close + 1..]));
        }
        from = open + 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

    use super::*;
    use crate::level::data::{LEVELS, LevelData};
    use crate::level::world::WORLDS;
    use crate::map::species;

    /// The slots a template fills, by argument.
    fn slots(template: &str) -> BTreeSet<usize> {
        let (mut found, mut next, mut rest) = (BTreeSet::new(), 0, template);
        while let Some((_, slot, after)) = split_slot(rest) {
            found.insert(slot.unwrap_or_else(|| {
                next += 1;
                next - 1
            }));
            rest = after;
        }
        found
    }

    /// Whether `key` is one of the counted forms of `counted` (`title.progress.one` of
    /// `title.progress`).
    fn form_of(key: &str, counted: &BTreeSet<String>) -> bool {
        counted.iter().any(|c| key.strip_prefix(c.as_str()).is_some_and(|f| f.starts_with('.')))
    }

    /// The keys written in the code: the literal in each `t("…")`, `tf("…")` and `n_("…")`, and
    /// apart, the counted ones of `tp("…")`.
    fn code_keys(dir: &Path, keys: &mut BTreeSet<String>, counted: &mut BTreeSet<String>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                code_keys(&path, keys, counted);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") || path.ends_with("i18n.rs") {
                continue;
            }
            let source = std::fs::read_to_string(&path).unwrap();
            for line in source.lines().filter(|l| !l.trim_start().starts_with("//")) {
                for call in ["t(\"", "tf(\"", "n_(\"", "tp(\""] {
                    for (at, _) in line.match_indices(call) {
                        // `t(` and not `at(`.
                        if line[..at]
                            .chars()
                            .last()
                            .is_some_and(|c| c.is_alphanumeric() || c == '_')
                        {
                            continue;
                        }
                        let key = literal(&line[at + call.len()..]);
                        if call == "tp(\"" {
                            counted.insert(key);
                        } else {
                            keys.insert(key);
                        }
                    }
                }
            }
        }
    }

    /// A Rust string literal's content, from just after its opening quote.
    fn literal(from: &str) -> String {
        let (mut out, mut chars) = (String::new(), from.chars());
        while let Some(c) = chars.next() {
            match c {
                '"' => break,
                '\\' => out.extend(chars.next()),
                c => out.push(c),
            }
        }
        out
    }

    /// The keys in the data: levels, worlds and their bosses, species.
    fn data_keys(keys: &mut BTreeSet<String>) {
        for level in LEVELS {
            let data: LevelData = ron::from_str(level).unwrap();
            keys.insert(data.name);
            keys.extend(data.notes.iter().map(|n| n.text().to_string()));
            keys.extend(data.challenge.map(|c| c.text));
            if let Some(choice) = data.choice {
                keys.insert(choice.prompt);
                keys.extend(choice.options.into_iter().map(|(_, label)| label));
            }
            keys.extend(data.places.into_iter().map(|p| p.name));
        }
        for world in WORLDS.iter() {
            keys.extend([world.name.clone(), world.defends.clone()]);
            if let Some(event) = &world.event {
                let said =
                    [&event.announce, &event.during, &event.refuse_next, &event.refuse_after];
                keys.extend(said.map(String::clone));
            }
            if let Some(boss) = &world.boss {
                keys.insert(boss.name.clone());
                keys.extend(boss.phases.iter().map(|p| p.name.clone()));
            }
        }
        let species = species::load();
        // The Lexique's descriptions: `<label>.text`, built, not written out in the code.
        for tab in crate::campaign::lexique::Tab::ALL {
            keys.extend(crate::campaign::lexique::entries(tab, &species).iter().map(|e| e.text()));
        }
        keys.extend(species.into_iter().map(|s| s.label));
    }

    #[test]
    fn numbers_read_with_the_decimal_mark() {
        assert_eq!(number(0.8), "0,8");
        assert_eq!(number(3.0), "3");
        assert_eq!(number(1.25), "1,2");
        assert_eq!(number(f32::INFINITY), "—");
    }

    #[test]
    fn every_language_has_every_key() {
        let (mut used, mut counted) = (BTreeSet::new(), BTreeSet::new());
        code_keys(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut used, &mut counted);
        data_keys(&mut used);
        used.extend([
            "language.name".into(),
            "language.decimal".into(),
            "common.unreachable".into(),
        ]);
        let french = &TABLES[0];
        let missing: Vec<&String> = used.iter().filter(|k| !french.contains_key(*k)).collect();
        let forms: Vec<String> =
            counted.iter().flat_map(|k| [format!("{k}.one"), format!("{k}.other")]).collect();
        let forms_missing: Vec<&String> =
            forms.iter().filter(|k| !french.contains_key(*k)).collect();
        let unused: Vec<&String> =
            french.keys().filter(|k| !used.contains(*k) && !form_of(k, &counted)).collect();
        assert!(
            missing.is_empty() && forms_missing.is_empty() && unused.is_empty(),
            "i18n/fr.ron:\n  missing: {missing:#?}\n  counted forms missing: {forms_missing:#?}\n  no longer used: {unused:#?}"
        );
        for (k, language) in LANGUAGES.iter().enumerate().skip(1) {
            let table = &TABLES[k];
            let missing: Vec<&String> =
                french.keys().filter(|key| !table.contains_key(*key)).collect();
            // A language may have more counted forms than French (`.few`, `.many`).
            let extra: Vec<&String> = table
                .keys()
                .filter(|key| !french.contains_key(*key) && !form_of(key, &counted))
                .collect();
            let empty: Vec<&String> =
                table.iter().filter(|(_, l)| l.trim().is_empty()).map(|(k, _)| k).collect();
            let slips: Vec<&String> = table
                .iter()
                .filter(|(key, l)| french.get(*key).is_some_and(|fr| slots(fr) != slots(l)))
                .map(|(k, _)| k)
                .collect();
            assert!(
                missing.is_empty() && extra.is_empty() && empty.is_empty() && slips.is_empty(),
                "i18n/{}.ron, against fr.ron:\n  missing: {missing:#?}\n  not in fr.ron: {extra:#?}\n  empty: {empty:#?}\n  \
                 slots differ from the French: {slips:#?}",
                language.code
            );
        }
    }

    #[test]
    fn missing_lines_fall_back() {
        let en = index("en");
        assert_eq!(lookup("menu.settings", en), "Settings");
        assert_eq!(lookup("menu.settings", 0), "Réglages");
        assert_eq!(lookup("pas.une.cle", en), "pas.une.cle");
        assert_eq!(index("xx"), 0, "an unknown language is French");
        assert_eq!(name("en"), "English");
        assert_eq!(next("fr", 1), "en");
        assert_eq!(next("fr", -1), LANGUAGES[LANGUAGES.len() - 1].code);
        assert_eq!(french_key("place.", "le Pont du Bief"), Some("place.pont_du_bief"));
    }

    #[test]
    fn templates_fill_in_order_or_by_number() {
        assert_eq!(fill("{} sur {}", &[&1, &2]), "1 sur 2");
        assert_eq!(fill("{1} of {0}", &[&1, &2]), "2 of 1");
        assert_eq!(fill("Vague {}", &[]), "Vague ");
        assert_eq!(fill("{x} reste {}", &[&3]), "{x} reste 3");
    }

    #[test]
    fn counts_take_each_language_s_plural() {
        let (fr, en) = (LANGUAGES[0].plural, LANGUAGES[index("en")].plural);
        assert_eq!([fr(0), fr(1), fr(2)], ["one", "one", "other"]);
        assert_eq!([en(0), en(1), en(2)], ["other", "one", "other"]);
    }
}
