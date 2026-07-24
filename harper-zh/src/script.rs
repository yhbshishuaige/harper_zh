//! Character script helpers for Chinese / mixed-language detection.

/// Coarse script category for a single character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptKind {
    Cjk,
    Latin,
    Digit,
    Space,
    Punctuation,
    Other,
}

/// Classify a character for mixed Chinese–English handling.
pub fn classify_char(c: char) -> ScriptKind {
    if c.is_whitespace() {
        return ScriptKind::Space;
    }
    if c.is_ascii_digit() {
        return ScriptKind::Digit;
    }
    if is_cjk_char(c) {
        return ScriptKind::Cjk;
    }
    if c.is_ascii_alphabetic() {
        return ScriptKind::Latin;
    }
    if c.is_ascii_punctuation() || is_cjk_punctuation(c) {
        return ScriptKind::Punctuation;
    }
    ScriptKind::Other
}

/// Han / CJK Unified Ideographs (basic + common extensions).
pub fn is_cjk_char(c: char) -> bool {
    matches!(
        c,
        '\u{4E00}'..='\u{9FFF}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{F900}'..='\u{FAFF}'
    ) || {
        let u = c as u32;
        (0x20000..=0x2A6DF).contains(&u)
    }
}

pub fn is_cjk_punctuation(c: char) -> bool {
    matches!(
        c,
        '，' | '。' | '、' | '；' | '：' | '？' | '！' | '「' | '」' | '『' | '』'
            | '（' | '）' | '【' | '】' | '《' | '》' | '〈' | '〉' | '…' | '—'
            | '～' | '·' | '“' | '”' | '‘' | '’'
    )
}

/// True if the character is a CJK ideograph (not punctuation).
pub fn is_han(c: char) -> bool {
    matches!(c, '\u{4E00}'..='\u{9FFF}' | '\u{3400}'..='\u{4DBF}' | '\u{F900}'..='\u{FAFF}')
        || {
            let u = c as u32;
            (0x20000..=0x2A6DF).contains(&u)
        }
}

pub fn has_cjk(text: &str) -> bool {
    text.chars().any(is_cjk_char)
}

pub fn has_latin(text: &str) -> bool {
    text.chars().any(|c| c.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_mixed() {
        assert_eq!(classify_char('中'), ScriptKind::Cjk);
        assert_eq!(classify_char('A'), ScriptKind::Latin);
        assert_eq!(classify_char('1'), ScriptKind::Digit);
        assert_eq!(classify_char(' '), ScriptKind::Space);
        assert_eq!(classify_char('，'), ScriptKind::Punctuation);
    }
}
