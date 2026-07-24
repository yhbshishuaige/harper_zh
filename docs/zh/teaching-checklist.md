# 中文规则教研审核清单

用于新增 / 修改 `harper-zh/data/*.json` 时的人工把关。

## 一、收录前先问

1. **这是不是真实高频错误？**  
   学生作业、社交媒体、技术文档里是否常见？
2. **改正是否几乎唯一？**  
   若两种写法都合法（语境依赖），先不要做成简单替换。
3. **能否用多字短语表达？**  
   `登陆系统` 优于单独 `登陆`；`在见` 可接受，因为告别语境很稳。
4. **会不会误伤专名 / 术语？**  
   如地名、人名、品牌、古文引用。

## 二、条目质量

| 检查项 | 通过标准 |
|--------|----------|
| `bad` ≠ `good` | 必须 |
| `bad` 长度 | 建议 ≥ 2 个汉字（特例除外） |
| `message` | **英文**，说清 what is wrong / preferred form |
| 分类文件 | 同音→`homophone`；易混→`word_confusion` 等 |
| 重复 | 同一 `bad` 不要多条互相冲突 |

### message 写法示例（**中文**）

本 fork 的检查提示、问题类型、替换建议均使用 **中文**。

- 好：`疑似前后鼻音混淆：应为「今天早上」。`
- 好：`「在/再」混淆：告别应为「再见」。`
- 好：`修饰动词时用「地」：开心地跑。`
- 差：`错误`（无信息）
- 差：`Please fix`（非中文 / 无正误对照）

## 三、分类对照

| 错误类型 | 写入文件 | 规则名 |
|----------|----------|--------|
| 同音 / 近音 / 前后鼻音 | `homophone.json` | `ZhHomophoneSpell` |
| 成语错字 | `homophone.json` | `ZhHomophoneSpell` |
| 的 / 地 / 得 | `de_di_de.json` | `ZhDeDiDe` |
| 在/再、做/作、登录/登陆… | `word_confusion.json` | `ZhWordConfusion` |
| 的的、了了、非常非常非常 | `redundancy.json` | `ZhRedundancy` |
| 标点 | 改 `src/punctuation.rs` | `ZhPunctuation` |
| 中英空格 | 改 `src/cjk_english_spacing.rs` | `ZhCjkEnglishSpacing` |

## 四、前后鼻音关注点（教研）

用户关心的类型可优先补充：

| 类别 | 例子方向 |
|------|----------|
| in / ing | 今/精/经、心/星、频/平…（注意只收**整词错误**） |
| en / eng | 根/更、真/正、分/风… |
| an / ang | 安/昂、班/帮… |
| un / ong | 工/公/共、中/终、通/同… |
| ai / a 等 | 仅在有稳定错词对时收录 |

**原则：** 收「错词 → 正词」，不要做单字无条件替换。

## 五、测试样例模板

新增 ≥3 条规则时，建议在 PR / 笔记中附：

```text
【误】惊天早上吃饭了吗
【正】今天早上吃饭了吗
【规则】ZhHomophoneSpell

【误】他开心的跑回家
【正】他开心地跑回家
【规则】ZhDeDiDe

【误】请先登陆系统
【正】请先登录系统
【规则】ZhWordConfusion
```

本地验证：

```bash
echo '【你的误句】' | ./target/release/harper-cli lint --format compact
cargo test -p harper-zh
```

## 六、误报回归

改完后至少检查：

1. 正确句子不应被报：  
   `今天早上吃饭了吗？他开心地跑回家，跑得很快。`
2. 纯英文不应触发中文同音规则：  
   `This is pure English text.`
3. 中英混排空格：  
   - `使用React框架` → 应提示  
   - `使用 React 框架` → 不应提示

## 七、提交说明建议

```text
docs(zh)/rules: 补充 XX 类错误 N 条

- 新增：……
- 样例：……
- 已跑：cargo test -p harper-zh
```

若使用了 LLM / Agent，请在 PR 中注明（见仓库 `AGENT_POLICY.md`）。

## 八、暂缓收录

- 仅修辞偏好、无对错之分的表达
- 强依赖上下文的「都对」情况
- 繁简转换、方言用词（除非明确目标）
- 需要完整句法才能判断的错误（超出当前 MVP）
