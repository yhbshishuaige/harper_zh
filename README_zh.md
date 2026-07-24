<div id="header" align="center">
    <img src="logo.svg" width="400px" />
    <h1>Harper 中文版（harper_zh）</h1>
    <p>
      <a href="./README.md">English</a> ·
      <strong>简体中文</strong>
    </p>
</div>

**Harper** 是一款离线、注重隐私的语法检查器。  
本仓库是 [Automattic/harper](https://github.com/Automattic/harper) 的中文增强 fork，在保留英文检查能力的同时，增加了**中文拼写 / 常见语病 / 中英混排**的 MVP 支持。

> 上游定位：为英文写作提供「刚刚好」的语法检查。  
> 本 fork 目标：在不大改英文内核的前提下，让中文与中英混合文档也能被检查。

---

## 目录

- [本 fork 能做什么](#本-fork-能做什么)
- [快速开始](#快速开始)
- [中文规则一览](#中文规则一览)
- [使用示例](#使用示例)
- [如何扩展中文规则（教研）](#如何扩展中文规则教研)
- [项目结构](#项目结构)
- [设计说明](#设计说明)
- [当前边界（暂不做）](#当前边界暂不做)
- [文档索引](#文档索引)
- [上游与致谢](#上游与致谢)

---

## 本 fork 能做什么

| 能力 | 状态 | 说明 |
|------|------|------|
| 英文语法 / 拼写 | ✅ | 沿用上游 `harper-core` |
| 纯中文拼写错误 | ✅ | 同音、近音、前后鼻音等，如「惊天早上」→「今天早上」 |
| 常见中文语病 | ✅ | 的/地/得、在/再、做/作、以/已、象/像、须/需… |
| 中英混合文章 | ✅ | 中文规则 + 英文规则同时生效；中英之间缺空格可提示 |
| 接近母语级中文语法 | ❌ | **暂不实现** |

### 典型修正示例

| 原文 | 建议 | 规则 |
|------|------|------|
| 惊天早上吃饭了吗 | 今天早上吃饭了吗 | `ZhHomophoneSpell` |
| 开心的跑 | 开心地跑 | `ZhDeDiDe` |
| 跑的很快 | 跑得很快 | `ZhDeDiDe` |
| 我们明天在见 | 我们明天再见 | `ZhWordConfusion` |
| 做业 | 作业 | `ZhWordConfusion` |
| 登陆系统 | 登录系统 | `ZhWordConfusion` |
| 的的 / 了了 | 的 / 了 | `ZhRedundancy` |
| 使用React框架 | 使用 React 框架 | `ZhCjkEnglishSpacing` |
| 你好,世界 | 你好，世界 | `ZhPunctuation` |
| I has a problem | （英文主谓一致提示） | 上游英文规则 |

---

## 快速开始

### 环境要求

- Rust（建议 stable，本机已验证 `rustc 1.97+`）
- Git

### 克隆与编译

```bash
git clone git@github.com:yhbshishuaige/harper_zh.git
cd harper_zh

# 编译带中文规则的 CLI
cargo build -p harper-cli --release
```

### 检查一份文件

```bash
# 仓库自带中文样例
./target/release/harper-cli lint sample_zh.txt

# 紧凑输出（适合脚本 / CI）
./target/release/harper-cli lint sample_zh.txt --format compact

# JSON 输出
./target/release/harper-cli lint sample_zh.txt --format json

# 从标准输入检查
echo '惊天早上吃饭了吗' | ./target/release/harper-cli lint --format compact
```

### 只启用中文规则

```bash
./target/release/harper-cli lint your.txt \
  --only ZhHomophoneSpell,ZhDeDiDe,ZhWordConfusion,ZhRedundancy,ZhPunctuation,ZhCjkEnglishSpacing
```

### 关闭某条中文规则

```bash
# 例如不需要中英空格风格建议
./target/release/harper-cli lint your.txt --ignore ZhCjkEnglishSpacing
```

### 运行中文模块测试

```bash
cargo test -p harper-zh
```

---

## 中文规则一览

| 规则名 | 类型 | 数据 / 代码 | 作用 |
|--------|------|-------------|------|
| `ZhHomophoneSpell` | Spelling | `harper-zh/data/homophone.json` | 同音、近音、前后鼻音、成语错字 |
| `ZhDeDiDe` | Grammar | `harper-zh/data/de_di_de.json` | 「的 / 地 / 得」常见误用 |
| `ZhWordConfusion` | WordChoice | `harper-zh/data/word_confusion.json` | 在/再、做/作、登录/登陆… |
| `ZhRedundancy` | Repetition | `harper-zh/data/redundancy.json` | 多余重复 |
| `ZhPunctuation` | Punctuation | `harper-zh/src/punctuation.rs` | 省略号、中英标点混用 |
| `ZhCjkEnglishSpacing` | Style | `harper-zh/src/cjk_english_spacing.rs` | 汉字与英文之间宜加空格 |

**前后鼻音 / 同音**等拼写类错误主要落在 `homophone.json`，覆盖用户关心的类型包括：

- **in / ing**、**en / eng**
- **an / ang**
- **un / ong**
- 常见成语错字（如「再接再励」→「再接再厉」）

---

## 使用示例

`sample_zh.txt` 中的片段与预期提示：

```text
惊天早上吃饭了吗？          → 今天早上…
他开心的跑回家，跑的很快。  → 开心地跑 / 跑得很快
我们明天在见，做业做完了吗？→ 再见 / 作业
我以经好象必需要登陆系统。  → 已经 / 好像 / 必须要 / 登录
使用React框架写页面。。。   → 中英空格 / 省略号……
I has a problem…            → 英文主谓一致
```

实际命令：

```bash
./target/release/harper-cli lint sample_zh.txt --format compact
```

---

## 如何扩展中文规则（教研）

**推荐方式：改 JSON，不要先改 Rust。**

1. 打开对应规则表，例如 `harper-zh/data/homophone.json`
2. 在 `pairs` 数组中追加：

```json
{
  "bad": "惊天早上",
  "good": "今天早上",
  "message": "疑似前后鼻音混淆：应为「今天早上」。"
}
```

3. 验证：

```bash
cargo test -p harper-zh
cargo build -p harper-cli --release
echo '你的测试句' | ./target/release/harper-cli lint --format compact
```

### 降低误报的原则

1. **优先多字短语**：`登陆系统` 优于单独的 `登陆`
2. **避免正误都合法的歧义对**，或加长上下文再收录
3. **同类错误放同一文件**（同音进 `homophone.json`，易混词进 `word_confusion.json`）
4. 较长错误串会优先匹配

完整流程见：**[harper-zh/CONTRIBUTING_ZH.md](./harper-zh/CONTRIBUTING_ZH.md)**

---

## 项目结构

```text
harper_zh/
├── README.md                 # 英文说明（含 fork 简介）
├── README_zh.md              # 本文件：中文总览
├── sample_zh.txt             # 中文 / 中英混排样例
├── harper-zh/                # 中文检查 crate（本 fork 核心增量）
│   ├── README.md             # 中文模块说明
│   ├── CONTRIBUTING_ZH.md    # 规则贡献 / 教研指南
│   ├── data/                 # 规则表（JSON，编译期嵌入）
│   │   ├── index.json
│   │   ├── homophone.json
│   │   ├── de_di_de.json
│   │   ├── word_confusion.json
│   │   └── redundancy.json
│   └── src/
│       ├── lib.rs
│       ├── rules.rs          # 加载 JSON，注册规则
│       ├── pattern_linter.rs # 通用模式匹配
│       ├── punctuation.rs
│       ├── cjk_english_spacing.rs
│       └── script.rs
├── harper-core/              # 上游英文内核
├── harper-cli/               # 命令行（已 merge 中文规则）
└── harper-ls/                # Language Server（已 merge 中文规则）
```

---

## 设计说明

1. **英文管线不动核心语义**：中文规则作为额外 `LintGroup` 合并进 CLI / LS。
2. **中文按「原文子串」匹配**：Harper 英文分词会把每个汉字标成 `Unlintable`，因此中文规则扫描**字符级原文**，而不是英文词 token。
3. **中英混合**：  
   - 中文片段 → `harper-zh`  
   - 拉丁字母片段 → `harper-core`  
   - 结果合并展示  
4. **规则数据与逻辑分离**：短语类规则在 JSON 中维护，方便教研批量增删。

---

## 当前边界（暂不做）

- 完整中文句法分析 / 依存句法
- 「接近母语级」修辞与语体润色
- 繁简转换与两岸用词全面规范化
- 开放域错别字模型（如大规模分词 + 语言模型；后续可讨论 jieba 等方案）

---

## 文档索引

| 文档 | 说明 |
|------|------|
| [README_zh.md](./README_zh.md) | 中文总览（本页） |
| [README.md](./README.md) | 英文总览 + 上游信息 |
| [harper-zh/README.md](./harper-zh/README.md) | 中文模块说明 |
| [harper-zh/CONTRIBUTING_ZH.md](./harper-zh/CONTRIBUTING_ZH.md) | 如何贡献 / 维护中文规则 |
| [sample_zh.txt](./sample_zh.txt) | 可直接 lint 的样例文本 |
| [上游架构说明](https://writewithharper.com/docs/contributors/architecture) | Harper 英文内核架构 |
| [AGENT_POLICY.md](./AGENT_POLICY.md) | 对 LLM / Agent 贡献 PR 的政策 |

---

## 上游与致谢

- 上游项目：[Automattic/harper](https://github.com/Automattic/harper)
- 本 fork：[yhbshishuaige/harper_zh](https://github.com/yhbshishuaige/harper_zh)
- 中文能力由 crate **`harper-zh`** 提供，并接入 `harper-cli` / `harper-ls`

Harper 原项目 logo 由 [Lukas Werner](https://lukaswerner.com/) 设计。

---

## 许可证

与上游一致，见 [LICENSE](./LICENSE)（Apache-2.0）。
