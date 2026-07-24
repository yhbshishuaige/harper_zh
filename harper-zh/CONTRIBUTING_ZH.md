# 中文规则贡献指南（教研用）

> 仓库中文总览：[../README_zh.md](../README_zh.md) · 模块说明：[README.md](./README.md)  
> 教研清单：[../docs/zh/teaching-checklist.md](../docs/zh/teaching-checklist.md) · 规则目录：[../docs/zh/rules-catalog.md](../docs/zh/rules-catalog.md)

本 fork 的中文检查在 `harper-zh` 中。**扩展规则优先改 JSON，不必改 Rust 匹配逻辑。**

## 目录

```
harper-zh/
  data/
    index.json           # 加载哪些规则表
    homophone.json       # 同音 / 前后鼻音 / 成语错字  → ZhHomophoneSpell
    de_di_de.json        # 的 / 地 / 得                 → ZhDeDiDe
    word_confusion.json  # 在/再、做/作…               → ZhWordConfusion
    redundancy.json      # 重复用词                     → ZhRedundancy
  src/
    pattern_linter.rs    # 通用字符串匹配
    cjk_english_spacing.rs
    punctuation.rs
    rules.rs             # 读 JSON 并注册到 LintGroup
```

## 加一条规则（推荐）

打开对应 JSON，在 `pairs` 数组里追加：

```json
{
  "bad": "惊天早上",
  "good": "今天早上",
  "message": "Possible nasal-final confusion: use “今天早上”."
}
```

字段说明：

| 字段 | 含义 |
|------|------|
| `bad` | 错误写法（会在全文中子串匹配） |
| `good` | 建议替换 |
| `message` | **展示给用户的说明（请用英文）** |

### 写法建议（降低误报）

1. **优先用多字短语**，少用单字。  
   - 好：`登陆系统` → `登录系统`  
   - 差：`登陆` → `登录`（可能误伤「登陆月球」）
2. **`bad` 不要等于 `good`**，无意义项会被过滤。
3. **歧义语境慎加**。若两种写法在不同语境都正确，要么加长上下文，要么先不收录。
4. **同类错误放同一文件**：  
   - 前后鼻音 / 同音 / 成语错字 → `homophone.json`  
   - 的地得 → `de_di_de.json`  
   - 易混词 → `word_confusion.json`  
   - 重复 → `redundancy.json`
5. 较长的 `bad` 会优先匹配（引擎按长度排序）。

## 验证

```bash
cd /path/to/harper_zh
cargo test -p harper-zh
cargo build -p harper-cli --release
echo '你的测试句子' | ./target/release/harper-cli lint --format compact
```

## 新增一整类规则表

1. 新建 `data/my_rules.json`（字段同现有文件：`name` / `kind` / `description` / `priority` / `pairs`）
2. 在 `data/index.json` 的 `sets` 里加上文件名
3. 在 `src/rules.rs` 的 `embedded_json_for` 和 `include_str!` 中注册该文件  
   （编译期嵌入，新增文件需要改这一处 Rust）

## 标点 / 中英空格

这两类是过程式规则，不在 JSON 里：

- `ZhPunctuation` → `src/punctuation.rs`
- `ZhCjkEnglishSpacing` → `src/cjk_english_spacing.rs`

## 当前目标边界

| 做 | 不做（暂缓） |
|----|----------------|
| 拼写级错字、常见语病短语 | 完整句法分析 |
| 的/地/得高频模式 | 母语级修辞润色 |
| 中英混排空格建议 | 方言/繁简深度规范 |

## 提交说明

PR / commit 建议写清：

- 新增/修改了哪些错误类型
- 附 1～3 条正误样例
- 是否考虑过误报

Agent / LLM 辅助的改动请在 PR 描述中注明（遵循仓库 `AGENT_POLICY.md`）。
