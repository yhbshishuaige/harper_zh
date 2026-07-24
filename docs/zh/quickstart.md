# 快速开始（中文）

## 1. 获取代码

```bash
git clone git@github.com:yhbshishuaige/harper_zh.git
cd harper_zh
```

## 2. 编译

```bash
cargo build -p harper-cli --release
```

完成后二进制位于：

```text
./target/release/harper-cli
```

## 3. 检查文本

```bash
# 检查样例文件
./target/release/harper-cli lint sample_zh.txt

# 紧凑输出
./target/release/harper-cli lint sample_zh.txt --format compact

# JSON（便于程序处理）
./target/release/harper-cli lint sample_zh.txt --format json

# 管道输入
echo '惊天早上吃饭了吗' | ./target/release/harper-cli lint --format compact
```

## 4. 常用开关

```bash
# 只开中文规则
./target/release/harper-cli lint file.txt \
  --only ZhHomophoneSpell,ZhDeDiDe,ZhWordConfusion,ZhRedundancy,ZhPunctuation,ZhCjkEnglishSpacing

# 关闭中英空格风格建议
./target/release/harper-cli lint file.txt --ignore ZhCjkEnglishSpacing

# 只统计错误数
./target/release/harper-cli lint file.txt -c --quiet
```

## 5. 改规则后验证

```bash
# 编辑例如：
#   harper-zh/data/homophone.json
#   harper-zh/data/word_confusion.json

cargo test -p harper-zh
cargo build -p harper-cli --release
```

## 6. 下一步

| 文档 | 内容 |
|------|------|
| [../README_zh.md](../../README_zh.md) | 中文总览 |
| [rules-catalog.md](./rules-catalog.md) | 全部短语规则目录 |
| [teaching-checklist.md](./teaching-checklist.md) | 教研审核清单 |
| [../../harper-zh/CONTRIBUTING_ZH.md](../../harper-zh/CONTRIBUTING_ZH.md) | 如何贡献规则 |


## 7. 自动修复与关闭小题大做

```bash
# 默认会忽略 Dashes（表格里的 --- 不当成破折号错误）
./target/release/harper-cli lint notes.md --format compact

# 若仍要检查 Dashes：
./target/release/harper-cli lint notes.md --no-default-ignore

# 直接改文件（只自动应用安全规则：中文/标点/中英空格）
# 拼写建议（SpellCheck）不会自动改，避免 gpt→get 一类误伤
./target/release/harper-cli lint notes.md --fix

# 自动修复会多轮应用安全规则；专有名词拼写（gpt/claude 等）不会自动改
# 若只想修中文、完全忽略英文拼写：
./target/release/harper-cli lint notes.md --fix --ignore SpellCheck

# 手动忽略更多规则
./target/release/harper-cli lint notes.md --ignore SpellCheck,ZhCjkEnglishSpacing
```
