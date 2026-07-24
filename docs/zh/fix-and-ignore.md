# 自动修复与规则忽略

## 常见抱怨与对应做法

| 你的感觉 | 做法 |
|----------|------|
| `---` 被改成破折号，小题大做 | **默认已忽略** `Dashes`；无需操作 |
| 只想看问题，不改文件 | `lint file.txt` |
| 直接改文件 | `lint file.txt --fix` |
| 模型名 gpt/claude 被报拼写 | 正常：`--fix` **不会**自动改拼写；可 `--ignore SpellCheck` |
| 中英空格提示太多 | `--ignore ZhCjkEnglishSpacing` 或 `--fix` 自动加空格 |

## 推荐命令

```bash
# 1) 只检查（默认已忽略 Dashes）
./bin/harper-cli-zh lint notes.md --format compact

# 2) 直接写回安全修复（中文语病 / 标点 / 中英空格）
#    多轮修复；拼写类不自动改，避免 gpt→get
./bin/harper-cli-zh lint notes.md --fix

# 3) 修完中文后，连拼写提示也不想看
./bin/harper-cli-zh lint notes.md --fix --ignore SpellCheck

# 4) 恢复检查 Dashes（表格 ---）
./bin/harper-cli-zh lint notes.md --no-default-ignore

# 5) 只跑中文规则
./bin/harper-cli-zh lint notes.md \
  --only ZhHomophoneSpell,ZhDeDiDe,ZhWordConfusion,ZhRedundancy,ZhPunctuation,ZhCjkEnglishSpacing
```

## `--fix` 会改什么 / 不会改什么

**会自动改（安全规则）：**

- `ZhHomophoneSpell` / `ZhDeDiDe` / `ZhWordConfusion` / `ZhRedundancy`
- `ZhPunctuation` / `ZhCjkEnglishSpacing`

**不会自动改：**

- `SpellCheck` 及多数英文拼写/用词建议（专有名词误伤风险高）

## 退出码

| 情况 | 退出码 |
|------|--------|
| 无问题 | 0 |
| `--fix` 后只剩拼写等建议性提示 | 0（并提示「安全项已处理完毕」） |
| 仍有需人工处理的问题 | 1 |
| 未使用 `--fix` 且存在问题 | 1 |

## 更新本地二进制

```bash
cd /path/to/harper_zh
cargo build -p harper-cli --release
cp -f target/release/harper-cli /home/loo/test/english/bin/harper-cli-zh
```
