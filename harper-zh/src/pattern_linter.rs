//! Generic string-pattern linter that scans the raw document source.
//!
//! Chinese text is tokenized as per-character `Unlintable` in Harper's English lexer,
//! so Chinese rules match against the character source rather than English word tokens.

use harper_core::linting::{Lint, LintKind, Linter, Suggestion};
use harper_core::{Document, Span};

/// One replaceable pattern: bad substring → good substring.
#[derive(Debug, Clone)]
pub struct PatternPair {
    pub bad: String,
    pub good: String,
    pub message: String,
}

/// A named set of pattern pairs that becomes one Harper rule.
#[derive(Debug, Clone)]
pub struct PatternRuleSet {
    pub name: String,
    pub description: String,
    pub lint_kind: LintKind,
    pub priority: u8,
    pub pairs: Vec<PatternPair>,
}

/// Linter that finds all non-overlapping occurrences of bad patterns in document source.
///
/// Longer patterns are matched first to reduce partial-match issues.
#[derive(Debug, Clone)]
pub struct ChinesePatternLinter {
    description: String,
    lint_kind: LintKind,
    priority: u8,
    /// Sorted by pattern length (char count) descending.
    pairs: Vec<PatternPair>,
}

impl ChinesePatternLinter {
    pub fn from_rule_set(set: PatternRuleSet) -> Self {
        let mut pairs = set.pairs;
        pairs.sort_by(|a, b| b.bad.chars().count().cmp(&a.bad.chars().count()));
        Self {
            description: set.description,
            lint_kind: set.lint_kind,
            priority: set.priority,
            pairs,
        }
    }
}

impl Linter for ChinesePatternLinter {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let source = document.get_source();
        if source.is_empty() || self.pairs.is_empty() {
            return Vec::new();
        }

        let text: String = source.iter().collect();
        let char_index = build_char_byte_index(&text);

        let mut used = vec![false; source.len()];
        let mut lints = Vec::new();

        for pair in &self.pairs {
            if pair.bad.is_empty() || pair.bad == pair.good {
                continue;
            }
            let bad_len = pair.bad.chars().count();
            if bad_len == 0 {
                continue;
            }

            let mut search_from = 0usize;
            while let Some(byte_pos) = text[search_from..].find(&pair.bad) {
                let abs_byte = search_from + byte_pos;
                let start_char = byte_to_char_index(&char_index, abs_byte);
                let end_char = start_char + bad_len;

                if end_char > source.len() {
                    break;
                }

                if used[start_char..end_char].iter().any(|&u| u) {
                    search_from = abs_byte + pair.bad.len().max(1);
                    continue;
                }

                for slot in &mut used[start_char..end_char] {
                    *slot = true;
                }

                lints.push(Lint {
                    span: Span::new(start_char, end_char),
                    lint_kind: self.lint_kind,
                    suggestions: vec![Suggestion::ReplaceWith(pair.good.chars().collect())],
                    message: pair.message.clone(),
                    priority: self.priority,
                });

                search_from = abs_byte + pair.bad.len().max(1);
            }
        }

        lints.sort_by_key(|l| l.span.start);
        lints
    }

    fn description(&self) -> &str {
        &self.description
    }
}

fn build_char_byte_index(text: &str) -> Vec<(usize, usize)> {
    text.char_indices()
        .enumerate()
        .map(|(ci, (bi, _))| (bi, ci))
        .collect()
}

fn byte_to_char_index(index: &[(usize, usize)], byte: usize) -> usize {
    match index.binary_search_by_key(&byte, |(b, _)| *b) {
        Ok(i) => index[i].1,
        Err(i) => {
            if i == 0 {
                0
            } else {
                index[i - 1].1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use harper_core::linting::LintKind;
    use harper_core::{Document, spell::FstDictionary};

    fn lint_text(text: &str, pairs: Vec<PatternPair>) -> Vec<Lint> {
        let dict = FstDictionary::curated();
        let doc = Document::new_plain_english(text, &dict);
        let set = PatternRuleSet {
            name: "Test".into(),
            description: "test".into(),
            lint_kind: LintKind::Spelling,
            priority: 50,
            pairs,
        };
        let mut linter = ChinesePatternLinter::from_rule_set(set);
        linter.lint(&doc)
    }

    #[test]
    fn finds_simple_pair() {
        let text = "惊天早上吃饭了吗";
        let lints = lint_text(
            text,
            vec![PatternPair {
                bad: "惊天早上".into(),
                good: "今天早上".into(),
                message: "应为今天早上".into(),
            }],
        );
        assert_eq!(lints.len(), 1);
        let chars: Vec<char> = text.chars().collect();
        assert_eq!(lints[0].get_str(&chars), "惊天早上");
    }

    #[test]
    fn prefers_longer_match() {
        let lints = lint_text(
            "非常非常非常好",
            vec![
                PatternPair {
                    bad: "非常".into(),
                    good: "很".into(),
                    message: "short".into(),
                },
                PatternPair {
                    bad: "非常非常非常".into(),
                    good: "非常".into(),
                    message: "long".into(),
                },
            ],
        );
        assert_eq!(lints.len(), 1);
        assert_eq!(lints[0].message, "long");
    }
}
