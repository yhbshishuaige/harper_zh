//! Chinese language support for Harper (`harper_zh` fork).
//!
//! Scope (MVP):
//! - Pure Chinese spelling confusions (homophones, nasal finals, 的/地/得, etc.)
//! - Common Chinese usage errors
//! - Chinese–English mixed text (spacing style + English still handled by core)
//!
//! Out of scope for now: near-native full Chinese grammar.

mod cjk_english_spacing;
mod pattern_linter;
mod punctuation;
mod rules;
mod script;

pub use pattern_linter::{ChinesePatternLinter, PatternPair, PatternRuleSet};
pub use rules::{chinese_lint_group, load_builtin_rule_sets};
pub use script::{ScriptKind, classify_char, has_cjk, has_latin};

use harper_core::linting::LintGroup;

/// Build a [`LintGroup`] containing all built-in Chinese rules (enabled by default).
pub fn lint_group() -> LintGroup {
    chinese_lint_group()
}

/// Merge all Chinese rules into an existing [`LintGroup`] (typically the curated English group).
pub fn extend_lint_group(group: &mut LintGroup) {
    group.merge_from(chinese_lint_group());
}
