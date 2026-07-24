# 中文文档索引

本目录存放 **harper_zh** fork 的中文使用与教研文档。

## 文档列表

| 文档 | 说明 |
|------|------|
| [../../README_zh.md](../../README_zh.md) | **中文总览**（推荐入口） |
| [quickstart.md](./quickstart.md) | 安装、编译、常用命令 |
| [rules-catalog.md](./rules-catalog.md) | 全部短语规则目录（由 JSON 汇总） |
| [teaching-checklist.md](./teaching-checklist.md) | 教研审核清单 |
| [../../harper-zh/README.md](../../harper-zh/README.md) | `harper-zh` 模块说明 |
| [../../harper-zh/CONTRIBUTING_ZH.md](../../harper-zh/CONTRIBUTING_ZH.md) | 如何用 JSON 扩展规则 |
| [../../sample_zh.txt](../../sample_zh.txt) | 可直接 lint 的样例 |

## 英文文档

| 文档 | 说明 |
|------|------|
| [../../README.md](../../README.md) | 英文总览（含中英文切换） |
| [上游文档站点](https://writewithharper.com/docs/contributors/introduction) | Harper 贡献与架构 |

## 三条最快路径

1. **我只想用：** 读 [quickstart.md](./quickstart.md)  
2. **我要加规则：** 读 [../../harper-zh/CONTRIBUTING_ZH.md](../../harper-zh/CONTRIBUTING_ZH.md) + [teaching-checklist.md](./teaching-checklist.md)  
3. **我要浏览已有规则：** 打开 [rules-catalog.md](./rules-catalog.md)

## 重新生成规则目录

当 `harper-zh/data/*.json` 有较大变更时，可重新生成目录页（仓库内脚本式维护）：

```bash
# 在仓库根目录，用你习惯的方式根据 data/*.json 更新
# docs/zh/rules-catalog.md
# （当前由维护者在更新规则时一并刷新）
```
