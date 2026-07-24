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
| `ZhHomophoneSpell` | Spelling | 同音/近音/前后鼻音错字 + 成语错字，如「惊天早上」→「今天早上」、「再接再励」→「再接再厉」 |
| `ZhDeDiDe` | Grammar | 「的/地/得」常见误用 |
| `ZhWordConfusion` | WordChoice | 在/再、做/作、以/已、象/像、须/需、登录/登陆、帐/账… |
| `ZhRedundancy` | Repetition | 的的、了了、非常非常非常… |
| `ZhPunctuation` | Punctuation | 。。。、中英标点混用 |
| `ZhCjkEnglishSpacing` | Style | `使用React` → `使用 React` |

Homophone coverage groups (see `src/rules.rs`):

- **in / ing**、**en / eng**（前后鼻音）
- **an / ang**
- **un / ong**（用户需求）
- 成语/固定搭配错字（再接再厉、迫不及待…）

## Usage

```bash
cargo build -p harper-cli --release
./target/release/harper-cli lint sample_zh.txt
./target/release/harper-cli lint sample_zh.txt --format compact
```

Only Chinese rules:

```bash
./target/release/harper-cli lint file.txt \
  --only ZhHomophoneSpell,ZhDeDiDe,ZhWordConfusion,ZhRedundancy,ZhPunctuation,ZhCjkEnglishSpacing
```

Disable one rule:

```bash
./target/release/harper-cli lint file.txt --ignore ZhCjkEnglishSpacing
```

## Extending rules

Edit `src/rules.rs`:

```rust
pair("错词", "正词", "提示信息"),
```

in one of:

- `homophone_pairs()` — 拼写/同音/前后鼻音/成语
- `de_di_de_pairs()` — 的/地/得
- `word_confusion_pairs()` — 易混词
- `redundancy_pairs()` — 重复

Then:

```bash
cargo test -p harper-zh
cargo build -p harper-cli --release
```

**Tips for low false positives:** prefer multi-character phrases (`登陆系统`) over single ambiguous characters; avoid pairs that are both valid in different contexts unless the bad form is clearly rare.

## Design notes

Harper’s English lexer marks each CJK character as `Unlintable`, so Chinese rules match against the **raw character source**, not English word tokens. English segments in mixed documents continue to use the existing `harper-core` pipeline.

## Tests

```bash
cargo test -p harper-zh
```
