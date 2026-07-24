//! Built-in Chinese rule tables and LintGroup assembly.
//!
//! Rule pairs live in `data/*.json` (embedded at compile time) so teachers and
//! contributors can extend them without touching Rust matching logic.
//!
//! Lint **messages and rule descriptions are English** (consistent with upstream Harper).
//! The `bad` / `good` forms remain Chinese text being corrected.

use harper_core::linting::{LintGroup, LintKind};
use serde::Deserialize;

use crate::cjk_english_spacing::CjkEnglishSpacing;
use crate::pattern_linter::{ChinesePatternLinter, PatternPair, PatternRuleSet};
use crate::punctuation::ChinesePunctuation;

const DATA_INDEX: &str = include_str!("../data/index.json");
const DATA_HOMOPHONE: &str = include_str!("../data/homophone.json");
const DATA_DE_DI_DE: &str = include_str!("../data/de_di_de.json");
const DATA_WORD_CONFUSION: &str = include_str!("../data/word_confusion.json");
const DATA_REDUNDANCY: &str = include_str!("../data/redundancy.json");

#[derive(Debug, Deserialize)]
struct DataIndex {
    sets: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct JsonRuleSet {
    name: String,
    kind: String,
    description: String,
    priority: u8,
    pairs: Vec<JsonPair>,
}

#[derive(Debug, Deserialize)]
struct JsonPair {
    bad: String,
    good: String,
    message: String,
}

fn lint_kind_from_str(s: &str) -> LintKind {
    match s {
        "Spelling" => LintKind::Spelling,
        "Grammar" => LintKind::Grammar,
        "WordChoice" => LintKind::WordChoice,
        "Repetition" => LintKind::Repetition,
        "Punctuation" => LintKind::Punctuation,
        "Style" => LintKind::Style,
        "Usage" => LintKind::Usage,
        "Typo" => LintKind::Typo,
        "Redundancy" => LintKind::Redundancy,
        _ => LintKind::Miscellaneous,
    }
}

fn parse_rule_set(json: &str) -> PatternRuleSet {
    let raw: JsonRuleSet = serde_json::from_str(json).expect("invalid chinese rule JSON");
    let pairs = raw
        .pairs
        .into_iter()
        .filter(|p| p.bad != p.good && !p.bad.is_empty())
        .map(|p| PatternPair {
            bad: p.bad,
            good: p.good,
            message: p.message,
        })
        .collect();
    PatternRuleSet {
        name: raw.name,
        description: raw.description,
        lint_kind: lint_kind_from_str(&raw.kind),
        priority: raw.priority,
        pairs,
    }
}

fn embedded_json_for(file: &str) -> &'static str {
    match file {
        "homophone.json" => DATA_HOMOPHONE,
        "de_di_de.json" => DATA_DE_DI_DE,
        "word_confusion.json" => DATA_WORD_CONFUSION,
        "redundancy.json" => DATA_REDUNDANCY,
        other => panic!("unknown chinese rule data file: {other}"),
    }
}

/// Load all built-in pattern rule sets from embedded JSON.
pub fn load_builtin_rule_sets() -> Vec<PatternRuleSet> {
    let index: DataIndex = serde_json::from_str(DATA_INDEX).expect("invalid data/index.json");
    index
        .sets
        .iter()
        .map(|name| parse_rule_set(embedded_json_for(name)))
        .collect()
}

/// Build a [`LintGroup`] with all Chinese rules enabled by default.
pub fn chinese_lint_group() -> LintGroup {
    let mut group = LintGroup::empty();

    for set in load_builtin_rule_sets() {
        let name = set.name.clone();
        let linter = ChinesePatternLinter::from_rule_set(set);
        group.add(name.clone(), linter);
        group.config.set_rule_enabled(name, true);
    }

    group.add("ZhPunctuation", ChinesePunctuation::new());
    group.config.set_rule_enabled("ZhPunctuation", true);

    group.add("ZhCjkEnglishSpacing", CjkEnglishSpacing::new());
    group.config.set_rule_enabled("ZhCjkEnglishSpacing", true);

    group
}

#[cfg(test)]
mod tests {
    use super::*;
    use harper_core::{Document, spell::FstDictionary};

    fn lint_with_group(text: &str) -> Vec<(String, String)> {
        let dict = FstDictionary::curated();
        let doc = Document::new_plain_english(text, &dict);
        let mut group = chinese_lint_group();
        let named = group.organized_lints(&doc);
        let mut out = Vec::new();
        for (rule, lints) in named {
            for lint in lints {
                out.push((rule.clone(), lint.message.clone()));
            }
        }
        out
    }

    fn set_by_name(name: &str) -> PatternRuleSet {
        load_builtin_rule_sets()
            .into_iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("missing rule set {name}"))
    }

    #[test]
    fn catches_user_sample_homophone() {
        let hits = lint_with_group("惊天早上吃饭了吗");
        assert!(
            hits.iter().any(|(_, m)| m.contains("今天早上")),
            "expected 惊天早上→今天早上, got {:?}",
            hits
        );
    }

    #[test]
    fn catches_de_di() {
        let hits = lint_with_group("他开心的跑回家");
        assert!(hits.iter().any(|(r, _)| r == "ZhDeDiDe"), "{:?}", hits);
    }

    #[test]
    fn catches_zai_jian() {
        let hits = lint_with_group("我们明天在见");
        assert!(hits.iter().any(|(_, m)| m.contains("再见")), "{:?}", hits);
    }

    #[test]
    fn catches_mixed_spacing() {
        let hits = lint_with_group("使用React框架");
        assert!(
            hits.iter().any(|(r, _)| r == "ZhCjkEnglishSpacing"),
            "{:?}",
            hits
        );
    }

    #[test]
    fn english_still_separate() {
        let hits = lint_with_group("This is pure English text.");
        assert!(
            !hits.iter().any(|(r, _)| r == "ZhHomophoneSpell"),
            "{:?}",
            hits
        );
    }

    #[test]
    fn catches_chengyu() {
        let hits = lint_with_group("我们要再接再励");
        assert!(
            hits.iter().any(|(_, m)| m.contains("再接再厉")),
            "{:?}",
            hits
        );
    }

    #[test]
    fn catches_denglu() {
        let hits = lint_with_group("请先登陆系统");
        assert!(hits.iter().any(|(_, m)| m.contains("登录")), "{:?}", hits);
    }

    #[test]
    fn no_false_positive_on_correct_chinese() {
        let hits = lint_with_group("今天早上吃饭了吗？他开心地跑回家，跑得很快。");
        assert!(
            !hits
                .iter()
                .any(|(r, m)| r == "ZhHomophoneSpell" && m.contains("今天早上")),
            "should not flag correct 今天早上: {:?}",
            hits
        );
        assert!(
            !hits
                .iter()
                .any(|(r, m)| r == "ZhDeDiDe" && m.contains("开心地跑")),
            "{:?}",
            hits
        );
    }

    #[test]
    fn rule_tables_have_content() {
        assert!(set_by_name("ZhHomophoneSpell").pairs.len() >= 80);
        assert!(set_by_name("ZhDeDiDe").pairs.len() >= 40);
        assert!(set_by_name("ZhWordConfusion").pairs.len() >= 40);
        assert!(set_by_name("ZhRedundancy").pairs.len() >= 10);
    }

    #[test]
    fn index_lists_all_embedded_sets() {
        let sets = load_builtin_rule_sets();
        assert_eq!(sets.len(), 4);
        let names: Vec<_> = sets.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"ZhHomophoneSpell"));
        assert!(names.contains(&"ZhDeDiDe"));
        assert!(names.contains(&"ZhWordConfusion"));
        assert!(names.contains(&"ZhRedundancy"));
    }

    #[test]
    fn messages_are_english() {
        for set in load_builtin_rule_sets() {
            assert!(
                set.description.is_ascii()
                    || set.description.chars().next().unwrap().is_ascii_alphabetic()
                    || set.description.starts_with("Common")
                    || set.description.starts_with("Redundant"),
                "description should be English-led: {}",
                set.description
            );
            for p in &set.pairs {
                let m = &p.message;
                assert!(
                    m.starts_with("Possible")
                        || m.starts_with("Use ")
                        || m.starts_with("Confusion")
                        || m.starts_with("Redundant")
                        || m.starts_with("Idiom")
                        || m.starts_with("Prefer")
                        || m.starts_with("For ")
                        || m.starts_with("In ")
                        || m.starts_with("Fixed")
                        || m.chars().next().is_some_and(|c| c.is_ascii_alphabetic()),
                    "message should be English: {} ({})",
                    m,
                    p.bad
                );
            }
        }
    }
}
