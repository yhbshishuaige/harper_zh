//! Chinese punctuation style rules (ellipsis, mixed ASCII punctuation in CJK context).

use harper_core::linting::{Lint, LintKind, Linter, Suggestion};
use harper_core::{Document, Span};

use crate::script::is_han;

/// Flags common Chinese punctuation issues.
#[derive(Debug, Default, Clone)]
pub struct ChinesePunctuation {
    description: &'static str,
}

impl ChinesePunctuation {
    pub fn new() -> Self {
        Self {
            description: "中文标点与中英标点混用建议。",
        }
    }
}

impl Linter for ChinesePunctuation {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let source = document.get_source();
        if source.is_empty() {
            return Vec::new();
        }

        let mut lints = Vec::new();
        let text: String = source.iter().collect();
        let has_han = source.iter().copied().any(is_han);

        // 。。。 → ……
        let mut search = 0usize;
        while let Some(byte_pos) = text[search..].find("。。。") {
            let abs = search + byte_pos;
            let start = byte_to_char(&text, abs);
            lints.push(Lint {
                span: Span::new(start, start + 3),
                lint_kind: LintKind::Punctuation,
                suggestions: vec![Suggestion::ReplaceWith("……".chars().collect())],
                message: "中文省略号宜用「……」，而不是连续三个「。」。".into(),
                priority: 60,
            });
            search = abs + "。。。".len();
        }

        // ... in a document that contains Han
        if has_han {
            let mut search = 0usize;
            while let Some(byte_pos) = text[search..].find("...") {
                let abs = search + byte_pos;
                let start = byte_to_char(&text, abs);
                // Only flag if near Han (within 8 chars either side)
                let lo = start.saturating_sub(8);
                let hi = (start + 11).min(source.len());
                if source[lo..hi].iter().copied().any(is_han) {
                    lints.push(Lint {
                        span: Span::new(start, start + 3),
                        lint_kind: LintKind::Punctuation,
                        suggestions: vec![Suggestion::ReplaceWith("……".chars().collect())],
                        message: "中文语境中省略号宜用「……」。".into(),
                        priority: 70,
                    });
                }
                search = abs + 3;
            }
        }

        // ASCII comma between Han characters: 你好,世界
        let mut i = 0usize;
        while i + 2 < source.len() {
            if source[i + 1] == ',' && is_han(source[i]) && is_han(source[i + 2]) {
                lints.push(Lint {
                    span: Span::new(i + 1, i + 2),
                    lint_kind: LintKind::Punctuation,
                    suggestions: vec![Suggestion::ReplaceWith(vec!['，'])],
                    message: "中文句子中宜使用中文逗号「，」。".into(),
                    priority: 65,
                });
            }
            i += 1;
        }

        // ASCII period between Han: 你好.世界 (less common but useful)
        let mut i = 0usize;
        while i + 2 < source.len() {
            if source[i + 1] == '.'
                && is_han(source[i])
                && is_han(source[i + 2])
                // avoid matching ...
                && source.get(i + 2) != Some(&'.')
                && source.get(i) != Some(&'.')
            {
                lints.push(Lint {
                    span: Span::new(i + 1, i + 2),
                    lint_kind: LintKind::Punctuation,
                    suggestions: vec![Suggestion::ReplaceWith(vec!['。'])],
                    message: "中文句子中宜使用中文句号「。」。".into(),
                    priority: 65,
                });
            }
            i += 1;
        }

        lints.sort_by_key(|l| l.span.start);
        lints
    }

    fn description(&self) -> &str {
        self.description
    }
}

fn byte_to_char(text: &str, byte: usize) -> usize {
    text[..byte].chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use harper_core::{Document, spell::FstDictionary};

    fn lint(text: &str) -> Vec<Lint> {
        let dict = FstDictionary::curated();
        let doc = Document::new_plain_english(text, &dict);
        let mut l = ChinesePunctuation::new();
        l.lint(&doc)
    }

    #[test]
    fn flags_triple_period() {
        let lints = lint("他走了。。。");
        assert!(lints.iter().any(|l| l.message.contains("省略号")));
    }

    #[test]
    fn flags_ascii_comma() {
        let lints = lint("你好,世界");
        assert!(lints.iter().any(|l| l.message.contains("逗号")));
    }
}
