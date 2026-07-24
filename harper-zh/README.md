# harper-zh

[English summary below](#english-summary) · **简体中文**

`harper-zh` 是 [harper_zh](https://github.com/yhbshishuaige/harper_zh) fork 中的**中文检查模块**，为 [Harper](https://github.com/Automattic/harper) 增加中文与中英混排支持。

仓库总览（中文）：[../README_zh.md](../README_zh.md)  
中文文档中心：[../docs/zh/README.md](../docs/zh/README.md)  
规则贡献（教研）：[CONTRIBUTING_ZH.md](./CONTRIBUTING_ZH.md)  
规则目录：[../docs/zh/rules-catalog.md](../docs/zh/rules-catalog.md)

---

## 目标范围（MVP）

| 目标 | 状态 |
|------|------|
| 纯中文拼写错误（同音 / 前后鼻音等） | ✅ |
| 常见中文语病（的/地/得、在/再…） | ✅ |
| 中英混合文章 | ✅ |
| 接近母语级中文语法 | ❌ 暂不实现 |

---

## 规则列表

| 规则名 | 类型 | 数据或代码 |
|--------|------|------------|
| `ZhHomophoneSpell` | Spelling | [`data/homophone.json`](./data/homophone.json) |
| `ZhDeDiDe` | Grammar | [`data/de_di_de.json`](./data/de_di_de.json) |
| `ZhWordConfusion` | WordChoice | [`data/word_confusion.json`](./data/word_confusion.json) |
| `ZhRedundancy` | Repetition | [`data/redundancy.json`](./data/redundancy.json) |
| `ZhPunctuation` | Punctuation | [`src/punctuation.rs`](./src/punctuation.rs) |
| `ZhCjkEnglishSpacing` | Style | [`src/cjk_english_spacing.rs`](./src/cjk_english_spacing.rs) |

### 拼写类分组（homophone.json）

- **in / ing**、**en / eng**（前后鼻音）
- **an / ang**
- **un / ong**
- 成语 / 固定搭配错字（如「再接再励」→「再接再厉」）

---

## 使用方法

在仓库根目录：

```bash
# 编译 CLI（已自动合并中文规则）
cargo build -p harper-cli --release

# 检查样例
./target/release/harper-cli lint ../sample_zh.txt --format compact

# 仅中文规则
./target/release/harper-cli lint ../sample_zh.txt \
  --only ZhHomophoneSpell,ZhDeDiDe,ZhWordConfusion,ZhRedundancy,ZhPunctuation,ZhCjkEnglishSpacing

# 关闭中英空格风格建议
./target/release/harper-cli lint ../sample_zh.txt --ignore ZhCjkEnglishSpacing
```

### 在代码中使用

```rust
use harper_core::linting::LintGroup;
use harper_core::{Document, Dialect};
use harper_core::spell::FstDictionary;
use std::sync::Arc;

let dict = FstDictionary::curated();
let doc = Document::new_plain_english("惊天早上吃饭了吗", &dict);

let mut group = LintGroup::new_curated(dict, Dialect::American);
harper_zh::extend_lint_group(&mut group);

let lints = group.organized_lints(&doc);
```

或直接：

```rust
let mut group = harper_zh::lint_group();
```

---

## 扩展规则（教研推荐）

**优先改 JSON**，无需改匹配引擎。

1. 编辑 `data/` 下对应文件的 `pairs` 字段  
2. 运行测试：

```bash
cargo test -p harper-zh
```

条目格式：

```json
{
  "bad": "在见",
  "good": "再见",
  "message": "「在/再」混淆：告别应为「再见」。"
}
```

详细约定、误报规避、新增整表步骤：  
→ **[CONTRIBUTING_ZH.md](./CONTRIBUTING_ZH.md)**

---

## 目录结构

```text
harper-zh/
├── README.md                 # 本文件
├── CONTRIBUTING_ZH.md        # 中文规则贡献指南
├── Cargo.toml
├── data/                     # 规则表（编译期 include_str 嵌入）
│   ├── index.json
│   ├── homophone.json
│   ├── de_di_de.json
│   ├── word_confusion.json
│   └── redundancy.json
└── src/
    ├── lib.rs                # lint_group / extend_lint_group
    ├── rules.rs              # 加载 JSON 并注册
    ├── pattern_linter.rs     # 通用 bad→good 匹配
    ├── punctuation.rs
    ├── cjk_english_spacing.rs
    └── script.rs             # 汉字 / 拉丁文字判定
```

---

## 设计要点

1. 中文在 Harper 英文分词中多为 `Unlintable`，故短语规则匹配**原文 Unicode 字符序列**。  
2. 中英混排时：中文走本 crate，英文仍走 `harper-core`。  
3. `harper-cli` / `harper-ls` 通过 `harper_zh::extend_lint_group` 合并规则。  
4. 规则表与逻辑分离，方便教研批量维护。

---

## 测试

```bash
cargo test -p harper-zh
```

---

## English summary

`harper-zh` adds **Chinese MVP linting** to the harper_zh fork of Harper:

- Homophone / nasal-final spelling fixes  
- Common usage (的/地/得, 在/再, …)  
- CJK–English spacing + punctuation style  
- Full native Chinese grammar is **out of scope**

Rule pairs live in `data/*.json` (compile-time embedded). See [CONTRIBUTING_ZH.md](./CONTRIBUTING_ZH.md) and the repo-level [README_zh.md](../README_zh.md).
