//! Style rule: insert a space between CJK ideographs and Latin letters.
//!
//! Example: `使用React框架` → `使用 React 框架`

use harper_core::linting::{Lint, LintKind, Linter, Suggestion};
use harper_core::{Document, Span};

use crate::script::{is_han, ScriptKind, classify_char};

/// Suggests spaces between Chinese characters and Latin words in mixed text.
#[derive(Debug, Default, Clone)]
pub struct CjkEnglishSpacing {
    description: &'static str,
}

impl CjkEnglishSpacing {
    pub fn new() -> Self {
        Self {
            description: "CJK–English mixed text: prefer a space between Han characters and Latin letters (style).",
        }
    }
}

impl Linter for CjkEnglishSpacing {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let source = document.get_source();
        if source.len() < 2 {
            return Vec::new();
        }

        let mut lints = Vec::new();
        let mut i = 0usize;
        while i + 1 < source.len() {
            let a = source[i];
            let b = source[i + 1];
            let ka = classify_char(a);
            let kb = classify_char(b);

            // CJK + Latin (no space): insert space after CJK char
            let cjk_then_latin = is_han(a) && matches!(kb, ScriptKind::Latin);
            // Latin + CJK (no space): insert space after Latin run — flag at boundary
            let latin_then_cjk = matches!(ka, ScriptKind::Latin) && is_han(b);

            if cjk_then_latin || latin_then_cjk {
                // Span covers the two characters at the boundary for clear highlighting.
                lints.push(Lint {
                    span: Span::new(i, i + 2),
                    lint_kind: LintKind::Style,
                    suggestions: vec![Suggestion::ReplaceWith(vec![a, ' ', b])],
                    message: if cjk_then_latin {
                        "Prefer a space between a Chinese character and an English word.".into()
                    } else {
                        "Prefer a space between an English word and a Chinese character.".into()
                    },
                    priority: 80,
                });
                // Skip the Latin run so we don't spam one lint per letter.
                if cjk_then_latin {
                    i += 1;
                    while i < source.len() && classify_char(source[i]) == ScriptKind::Latin {
                        i += 1;
                    }
                    continue;
                } else {
                    // latin_then_cjk: advance past this boundary
                    i += 1;
                    continue;
                }
            }
            i += 1;
        }

        lints
    }

    fn description(&self) -> &str {
        self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use harper_core::{Document, spell::FstDictionary};

    fn lint(text: &str) -> Vec<Lint> {
        let dict = FstDictionary::curated();
        let doc = Document::new_plain_english(text, &dict);
        let mut l = CjkEnglishSpacing::new();
        l.lint(&doc)
    }

    #[test]
    fn flags_cjk_latin() {
        let lints = lint("使用React框架");
        assert!(!lints.is_empty());
        assert!(lints.iter().any(|l| l.message.to_lowercase().contains("space")));
    }

    #[test]
    fn ok_with_spaces() {
        let lints = lint("使用 React 框架");
        assert!(lints.is_empty());
    }
}
