<div id="header" align="center">
    <img src="logo.svg" width="400px" />
    <h1>Harper</h1>
    <p>
      <strong>English</strong> ·
      <a href="./README_zh.md">简体中文</a>
    </p>
</div>

> **This repository is the `harper_zh` fork** with Chinese MVP support.
> Chinese docs: **[README_zh.md](./README_zh.md)** · Rule curation: **[harper-zh/CONTRIBUTING_ZH.md](./harper-zh/CONTRIBUTING_ZH.md)**

[![Harper Binaries](https://github.com/automattic/harper/actions/workflows/binaries.yml/badge.svg)](https://github.com/automattic/harper/actions/workflows/binaries.yml)
[![Website](https://github.com/automattic/harper/actions/workflows/build_web.yml/badge.svg)](https://github.com/automattic/harper/actions/workflows/build_web.yml)
[![Checks](https://github.com/automattic/harper/actions/workflows/just_checks.yml/badge.svg)](https://github.com/automattic/harper/actions/workflows/just_checks.yml)
[![Crates.io](https://img.shields.io/crates/v/harper-ls)](https://crates.io/crates/harper-ls)
![NPM Version](https://img.shields.io/npm/v/harper.js)
![Downloads](https://img.shields.io/github/downloads/automattic/harper/total?label=Binary+Downloads)
![Obsidian Plugin Downloads](https://img.shields.io/github/downloads/automattic/harper-obsidian-plugin/total?label=Obsidian+Plugin+Downloads)

Harper is an English grammar checker designed to be _just right._
I created it after years of dealing with the shortcomings of the competition.

Grammarly was too expensive and too overbearing.
Its suggestions lacked context, and were often just plain _wrong_.
Not to mention: it's a privacy nightmare.
Everything you write with Grammarly is sent to their servers.
Their privacy policy claims they don't sell the data, but that doesn't mean they don't use it to train large language models and god knows what else.
Not only that, but the round-trip-time of the network request makes revising your work all the more tedious.

LanguageTool is great, if you have gigabytes of RAM to spare and are willing to download the ~16GB n-gram dataset.
Besides the memory requirements, I found LanguageTool too slow: it would take several seconds to lint even a moderate-size document.

That's why I created Harper: it is the grammar checker that fits my needs.
Not only does it take milliseconds to lint a document, take less than 1/50th of LanguageTool's memory footprint,
but it is also completely private.

Harper is even small enough to load via [WebAssembly.](https://writewithharper.com)

## Language Support

This fork (`harper_zh`) adds **Chinese (中文) MVP support** via the `harper-zh` crate:

- Pure Chinese spelling confusions (homophones / nasal finals), e.g. `惊天早上` → `今天早上`
- Common usage errors: 的/地/得, 在/再, 做/作, 以/已, 象/像, 须/需, …
- Chinese–English mixed text spacing style + full English checking on Latin segments

### Chinese documentation

| Doc | Description |
|-----|-------------|
| **[README_zh.md](./README_zh.md)** | 中文总览：安装、用法、规则、样例 |
| **[docs/zh/README.md](./docs/zh/README.md)** | 中文文档中心 |
| **[docs/zh/quickstart.md](./docs/zh/quickstart.md)** | 快速开始 |
| **[docs/zh/rules-catalog.md](./docs/zh/rules-catalog.md)** | 短语规则完整目录 |
| **[docs/zh/teaching-checklist.md](./docs/zh/teaching-checklist.md)** | 教研审核清单 |
| **[harper-zh/README.md](./harper-zh/README.md)** | `harper-zh` 模块说明 |
| **[harper-zh/CONTRIBUTING_ZH.md](./harper-zh/CONTRIBUTING_ZH.md)** | 教研向：如何用 JSON 扩展规则 |
| **[sample_zh.txt](./sample_zh.txt)** | 中文 / 中英混排样例 |

```bash
cargo build -p harper-cli --release
./target/release/harper-cli lint sample_zh.txt --format compact
cargo test -p harper-zh
```

Near-native full Chinese grammar is **not** in scope yet.

Upstream Harper currently only supports English, but the core is extensible to support other languages, so we welcome contributions that allow for other language support.

## Performance Issues

We consider long lint times bugs.
If you encounter any significant performance issues, please create an issue on the topic.

If you find a fix to any performance issue, we would appreciate the contribution.
Just please make sure to read [our contribution guidelines first.](https://writewithharper.com/docs/contributors/introduction)

## Links

- [Frequently Asked Questions](https://writewithharper.com/#faqs)
- [Obsidian Documentation](https://writewithharper.com/docs/integrations/obsidian)
- [`harper-ls` Documentation](https://writewithharper.com/docs/integrations/language-server)
- Supported Editors' Documentation
  - [Visual Studio Code](https://writewithharper.com/docs/integrations/visual-studio-code)
  - [Neovim](https://writewithharper.com/docs/integrations/neovim)
  - [Helix](https://writewithharper.com/docs/integrations/helix)
  - [Emacs](https://writewithharper.com/docs/integrations/emacs)
  - [Zed](https://writewithharper.com/docs/integrations/zed)
- [`harper.js` Documentation](https://writewithharper.com/docs/harperjs/introduction)
- [Official Discord Server](https://discord.com/invite/JBqcAaKrzQ)

## Huge Thanks

This project would not be possible without the hard work from those who [contribute](https://writewithharper.com/docs/contributors/introduction).

<a href="https://github.com/automattic/harper/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=automattic/harper" />
</a>

Harper's logo was designed by [Lukas Werner](https://lukaswerner.com/).
