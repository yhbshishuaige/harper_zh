# harper-zh

Chinese language checking for the [harper_zh](https://github.com/yhbshishuaige/harper_zh) fork of [Harper](https://github.com/Automattic/harper).

## Goals (MVP)

| Goal | Status |
|------|--------|
| Pure Chinese spelling confusions (homophones, nasal finals) | ✅ pattern rules |
| Common Chinese usage errors (的/地/得, 在/再, …) | ✅ pattern rules |
| Chinese–English mixed text | ✅ spacing style + English via `harper-core` |
| Near-native full Chinese grammar | ❌ out of scope |

## Rules

| Rule name | Kind | Description |
|-----------|------|-------------|
| `ZhHomophoneSpell` | Spelling | 同音/近音/前后鼻音错字，如「惊天早上」→「今天早上」 |
| `ZhDeDiDe` | Grammar | 「的/地/得」常见误用 |
| `ZhWordConfusion` | WordChoice | 在/再、做/作、以/已、象/像、须/需、登录/登陆… |
| `ZhRedundancy` | Repetition | 的的、了了、非常非常非常… |
| `ZhPunctuation` | Punctuation | 。。。、中英标点混用 |
| `ZhCjkEnglishSpacing` | Style | `使用React` → `使用 React` |

## Usage

After building the workspace CLI:

```bash
cargo build -p harper-cli --release
./target/release/harper-cli lint sample_zh.txt
```

Chinese rules are merged into the default lint group automatically.

Disable a Chinese rule:

```bash
harper-cli lint --ignore ZhCjkEnglishSpacing file.txt
```

Only Chinese rules:

```bash
harper-cli lint --only ZhHomophoneSpell,ZhDeDiDe,ZhWordConfusion,ZhRedundancy,ZhPunctuation,ZhCjkEnglishSpacing file.txt
```

## Design notes

Harper’s English lexer marks each CJK character as `Unlintable`, so Chinese rules match against the **raw character source**, not English word tokens. English segments in mixed documents continue to use the existing `harper-core` pipeline.

## Extending

Add pairs in `src/rules.rs` (`homophone_pairs`, `de_di_de_pairs`, `word_confusion_pairs`, `redundancy_pairs`), or add a new `Linter` under `src/` and register it in `chinese_lint_group()`.
