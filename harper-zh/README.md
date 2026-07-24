# harper-zh

Chinese language checking for the [harper_zh](https://github.com/yhbshishuaige/harper_zh) fork of [Harper](https://github.com/Automattic/harper).

## Goals (MVP)

| Goal | Status |
|------|--------|
| Pure Chinese spelling confusions (homophones, nasal finals) | ✅ |
| Common Chinese usage errors (的/地/得, 在/再, …) | ✅ |
| Chinese–English mixed text | ✅ |
| Near-native full Chinese grammar | ❌ out of scope |

## Rules

| Rule name | Kind | Data file |
|-----------|------|-----------|
| `ZhHomophoneSpell` | Spelling | `data/homophone.json` |
| `ZhDeDiDe` | Grammar | `data/de_di_de.json` |
| `ZhWordConfusion` | WordChoice | `data/word_confusion.json` |
| `ZhRedundancy` | Repetition | `data/redundancy.json` |
| `ZhPunctuation` | Punctuation | (code) `src/punctuation.rs` |
| `ZhCjkEnglishSpacing` | Style | (code) `src/cjk_english_spacing.rs` |

## Usage

```bash
cargo build -p harper-cli --release
./target/release/harper-cli lint sample_zh.txt --format compact
```

Only Chinese rules:

```bash
./target/release/harper-cli lint file.txt \
  --only ZhHomophoneSpell,ZhDeDiDe,ZhWordConfusion,ZhRedundancy,ZhPunctuation,ZhCjkEnglishSpacing
```

## Extending (教研)

**优先编辑 JSON**，见 [CONTRIBUTING_ZH.md](./CONTRIBUTING_ZH.md)。

```json
{ "bad": "在见", "good": "再见", "message": "「在/再」混淆：告别应为「再见」。" }
```

```bash
cargo test -p harper-zh
```

## Design notes

Harper’s English lexer marks each CJK character as `Unlintable`, so Chinese pattern rules match the **raw character source**. English segments still use `harper-core`.
