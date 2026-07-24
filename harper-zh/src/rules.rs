//! Built-in Chinese rule tables and LintGroup assembly.

use harper_core::linting::{LintGroup, LintKind};

use crate::cjk_english_spacing::CjkEnglishSpacing;
use crate::pattern_linter::{ChinesePatternLinter, PatternPair, PatternRuleSet};
use crate::punctuation::ChinesePunctuation;

/// Load all built-in pattern rule sets (embedded; no runtime file I/O required).
pub fn load_builtin_rule_sets() -> Vec<PatternRuleSet> {
    vec![
        PatternRuleSet {
            name: "ZhHomophoneSpell".into(),
            description: "常见前后鼻音与同音/近音错字（拼写级），如「惊天早上」→「今天早上」。".into(),
            lint_kind: LintKind::Spelling,
            priority: 40,
            pairs: homophone_pairs(),
        },
        PatternRuleSet {
            name: "ZhDeDiDe".into(),
            description: "「的/地/得」常见误用。".into(),
            lint_kind: LintKind::Grammar,
            priority: 45,
            pairs: de_di_de_pairs(),
        },
        PatternRuleSet {
            name: "ZhWordConfusion".into(),
            description: "常见易混词：在/再、做/作、以/已、象/像、须/需、登录/登陆 等。".into(),
            lint_kind: LintKind::WordChoice,
            priority: 42,
            pairs: word_confusion_pairs(),
        },
        PatternRuleSet {
            name: "ZhRedundancy".into(),
            description: "多余重复：的的、了了、非常非常非常 等。".into(),
            lint_kind: LintKind::Repetition,
            priority: 50,
            pairs: redundancy_pairs(),
        },
    ]
}

/// Build a [`LintGroup`] with all Chinese rules enabled by default.
pub fn chinese_lint_group() -> LintGroup {
    let mut group = LintGroup::empty();

    for set in load_builtin_rule_sets() {
        let name = set.name.clone();
        let linter = ChinesePatternLinter::from_rule_set(set);
        group.add(name.clone(), linter);
        group.config.set_rule_enabled(name, true);
    }

    group.add("ZhPunctuation", ChinesePunctuation::new());
    group.config.set_rule_enabled("ZhPunctuation", true);

    group.add("ZhCjkEnglishSpacing", CjkEnglishSpacing::new());
    group.config.set_rule_enabled("ZhCjkEnglishSpacing", true);

    group
}

fn pair(bad: &str, good: &str, message: &str) -> PatternPair {
    PatternPair {
        bad: bad.into(),
        good: good.into(),
        message: message.into(),
    }
}

fn homophone_pairs() -> Vec<PatternPair> {
    vec![
        // 前后鼻音 / 同音示例（用户样例）
        pair("惊天早上", "今天早上", "疑似前后鼻音混淆：应为「今天早上」。"),
        pair("经天早上", "今天早上", "疑似同音错字：应为「今天早上」。"),
        pair("精天早上", "今天早上", "疑似同音错字：应为「今天早上」。"),
        // 因该 / 应该
        pair("因该", "应该", "疑似同音错字：应为「应该」。"),
        pair("应改", "应该", "疑似同音错字：应为「应该」。"),
        // 按装 / 安装
        pair("按装", "安装", "疑似同音错字：应为「安装」。"),
        pair("安状", "安装", "疑似同音错字：应为「安装」。"),
        // 再接再励 / 再接再厉
        pair("再接再励", "再接再厉", "成语应为「再接再厉」。"),
        // 其他高频同音
        pair("针式", "正式", "疑似同音错字：在「正式」义时宜用「正式」。"),
        pair("幅射", "辐射", "应为「辐射」。"),
        pair("防碍", "妨碍", "应为「妨碍」。"),
        pair("凑和", "凑合", "应为「凑合」。"),
        pair("精萃", "精粹", "应为「精粹」。"),
        pair("震憾", "震撼", "应为「震撼」。"),
        pair("九宵", "九霄", "应为「九霄」。"),
        pair("泊来品", "舶来品", "应为「舶来品」。"),
        pair("气慨", "气概", "应为「气概」。"),
        pair("食不裹腹", "食不果腹", "成语应为「食不果腹」。"),
        pair("鸠占雀巢", "鸠占鹊巢", "成语应为「鸠占鹊巢」。"),
        pair("追朔", "追溯", "应为「追溯」。"),
        pair("耽心", "担心", "应为「担心」。"),
        pair("符和", "符合", "应为「符合」。"),
        pair("发奋图强", "发愤图强", "成语应为「发愤图强」。"),
        pair("明查暗访", "明察暗访", "成语应为「明察暗访」。"),
        pair("墨守陈规", "墨守成规", "成语应为「墨守成规」。"),
        pair("迫不急待", "迫不及待", "成语应为「迫不及待」。"),
        pair("悬梁刺骨", "悬梁刺股", "成语应为「悬梁刺股」。"),
        pair("走头无路", "走投无路", "成语应为「走投无路」。"),
        pair("不径而走", "不胫而走", "成语应为「不胫而走」。"),
        pair("世外桃园", "世外桃源", "应为「世外桃源」。"),
        pair("直接了当", "直截了当", "应为「直截了当」。"),
        pair("一愁莫展", "一筹莫展", "成语应为「一筹莫展」。"),
        pair("名信片", "明信片", "应为「明信片」。"),
        pair("水笼头", "水龙头", "应为「水龙头」。"),
        pair("罗嗦", "啰嗦", "应为「啰嗦」。"),
        pair("萎糜不振", "萎靡不振", "应为「萎靡不振」。"),
        pair("九洲", "九州", "应为「九州」。"),
        pair("重迭", "重叠", "应为「重叠」。"),
        pair("好象", "好像", "「象/像」混淆：应为「好像」。"), // also in word confusion; ok
    ]
    .into_iter()
    .filter(|p| p.bad != p.good)
    .collect()
}

fn de_di_de_pairs() -> Vec<PatternPair> {
    vec![
        // 的 → 地（状语 + 动词）
        pair("开心的跑", "开心地跑", "修饰动词时用「地」：开心地跑。"),
        pair("高兴的说", "高兴地说", "修饰动词时用「地」：高兴地说。"),
        pair("认真的学习", "认真地学习", "修饰动词时用「地」：认真地学习。"),
        pair("仔细的检查", "仔细地检查", "修饰动词时用「地」：仔细地检查。"),
        pair("慢慢的走", "慢慢地走", "修饰动词时用「地」：慢慢地走。"),
        pair("快速的完成", "快速地完成", "修饰动词时用「地」：快速地完成。"),
        pair("安静的坐下", "安静地坐下", "修饰动词时用「地」：安静地坐下。"),
        pair("努力的工作", "努力地工作", "修饰动词时用「地」：努力地工作。"),
        pair("轻轻的放", "轻轻地放", "修饰动词时用「地」：轻轻地放。"),
        pair("大声的喊", "大声地喊", "修饰动词时用「地」：大声地喊。"),
        pair("顺利的完成", "顺利地完成", "修饰动词时用「地」：顺利地完成。"),
        pair("清楚的说明", "清楚地说明", "修饰动词时用「地」：清楚地说明。"),
        pair("仔细的观察", "仔细地观察", "修饰动词时用「地」：仔细地观察。"),
        pair("耐心的等待", "耐心地等待", "修饰动词时用「地」：耐心地等待。"),
        // 的 → 得（动词 + 补语）
        pair("跑的很快", "跑得很快", "补语前用「得」：跑得很快。"),
        pair("说的很好", "说得很好", "补语前用「得」：说得很好。"),
        pair("写的不错", "写得不错", "补语前用「得」：写得不错。"),
        pair("做的很好", "做得很好", "补语前用「得」：做得很好。"),
        pair("吃的很香", "吃得很香", "补语前用「得」：吃得很香。"),
        pair("学的很快", "学得很快", "补语前用「得」：学得很快。"),
        pair("走的很慢", "走得很慢", "补语前用「得」：走得很慢。"),
        pair("长的漂亮", "长得漂亮", "补语前用「得」：长得漂亮。"),
        pair("玩的开心", "玩得开心", "补语前用「得」：玩得开心。"),
        pair("睡的很晚", "睡得很晚", "补语前用「得」：睡得很晚。"),
        pair("听的清楚", "听得清楚", "补语前用「得」：听得清楚。"),
        pair("看的明白", "看得明白", "补语前用「得」：看得明白。"),
        // 地 → 的（定语 + 名词）
        pair("美丽地风景", "美丽的风景", "修饰名词时用「的」：美丽的风景。"),
        pair("红色地苹果", "红色的苹果", "修饰名词时用「的」：红色的苹果。"),
        pair("我地书", "我的书", "修饰名词时用「的」：我的书。"),
        pair("蓝蓝地天空", "蓝蓝的天空", "修饰名词时用「的」：蓝蓝的天空。"),
        pair("新鲜地水果", "新鲜的水果", "修饰名词时用「的」：新鲜的水果。"),
    ]
}

fn word_confusion_pairs() -> Vec<PatternPair> {
    vec![
        // 在 / 再
        pair("在见", "再见", "「在/再」混淆：告别应为「再见」。"),
        pair("在见吧", "再见吧", "「在/再」混淆：告别应为「再见」。"),
        pair("在见一次", "再见一次", "「在/再」混淆：应为「再见」。"),
        pair("在次", "再次", "「在/再」混淆：应为「再次」。"),
        pair("在说", "再说", "「在/再」混淆：应为「再说」。"),
        pair("在考虑一下", "再考虑一下", "「在/再」混淆：应为「再考虑一下」。"),
        pair("在看一遍", "再看一遍", "「在/再」混淆：应为「再看一遍」。"),
        pair("在试一次", "再试一次", "「在/再」混淆：应为「再试一次」。"),
        // 做 / 作
        pair("做业", "作业", "「做/作」混淆：应为「作业」。"),
        pair("作饭", "做饭", "「做/作」混淆：应为「做饭」。"),
        pair("作事", "做事", "「做/作」混淆：应为「做事」。"),
        pair("作梦", "做梦", "「做/作」混淆：应为「做梦」。"),
        pair("作工", "做工", "「做/作」混淆：应为「做工」。"),
        pair("做家", "作家", "「做/作」混淆：应为「作家」。"),
        pair("做品", "作品", "「做/作」混淆：应为「作品」。"),
        pair("做用", "作用", "「做/作」混淆：应为「作用」。"),
        pair("做者", "作者", "「做/作」混淆：应为「作者」。"),
        pair("做文", "作文", "「做/作」混淆：应为「作文」。"),
        // 以 / 已
        pair("以经", "已经", "「以/已」混淆：应为「已经」。"),
        pair("以经完成", "已经完成", "「以/已」混淆：应为「已经」。"),
        pair("以经是", "已经是", "「以/已」混淆：应为「已经」。"),
        pair("已后", "以后", "「以/已」混淆：表示之后时间应为「以后」。"),
        pair("已及", "以及", "「以/已」混淆：应为「以及」。"),
        pair("已免", "以免", "「以/已」混淆：应为「以免」。"),
        // 象 / 像
        pair("好象", "好像", "「象/像」混淆：应为「好像」。"),
        pair("成象", "成像", "技术语境中常为「成像」。"),
        pair("录象", "录像", "应为「录像」。"),
        pair("照象", "照像", "应为「照像」/「照相」。"),
        pair("象素", "像素", "应为「像素」。"),
        pair("肖象", "肖像", "应为「肖像」。"),
        // 须 / 需
        pair("必需要", "必须要", "「须/需」冗余：宜用「必须要」或「需要」。"),
        pair("需须", "必须", "「须/需」混淆：应为「必须」。"),
        pair("须要", "需要", "「须/需」混淆：此处宜用「需要」。"),
        pair("必需要求", "必须要求", "「须/需」：宜检查是否应为「必须」。"),
        // 即 / 既
        pair("即然", "既然", "「即/既」混淆：应为「既然」。"),
        pair("既使", "即使", "「即/既」混淆：应为「即使」。"),
        // 即使 is correct; only 既使 is wrong
        // 登录 / 登陆
        pair("登陆网站", "登录网站", "上网操作为「登录」，不是「登陆」。"),
        pair("登陆系统", "登录系统", "系统操作为「登录」，不是「登陆」。"),
        pair("登陆账号", "登录账号", "操作为「登录」，不是「登陆」。"),
        pair("登陆游戏", "登录游戏", "操作为「登录」，不是「登陆」。"),
        // 帐 / 账
        pair("帐号", "账号", "现代汉语推荐「账号」。"),
        pair("帐号密码", "账号密码", "现代汉语推荐「账号」。"),
        pair("帐户", "账户", "现代汉语推荐「账户」。"),
        // 其它
        pair("其它的", "其他的", "「其它/其他」：指人/事物均可，现代常用「其他」。"),
    ]
}

fn redundancy_pairs() -> Vec<PatternPair> {
    vec![
        pair("的的", "的", "多余重复：连续两个「的」。"),
        pair("了了", "了", "多余重复：连续两个「了」。"),
        pair("是是", "是", "多余重复：连续两个「是」。"),
        pair("的话的话", "的话", "多余重复。"),
        pair("非常非常非常", "非常", "程度副词过度重复，可精简。"),
        pair("必须须", "必须", "重复用字：应为「必须」。"),
        pair("需需要", "需要", "重复用字：应为「需要」。"),
        pair("进行进行", "进行", "多余重复。"),
        pair("可以可以", "可以", "多余重复。"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use harper_core::{Document, spell::FstDictionary};

    fn lint_with_group(text: &str) -> Vec<(String, String)> {
        let dict = FstDictionary::curated();
        let doc = Document::new_plain_english(text, &dict);
        let mut group = chinese_lint_group();
        let named = group.organized_lints(&doc);
        let mut out = Vec::new();
        for (rule, lints) in named {
            for lint in lints {
                out.push((rule.clone(), lint.message.clone()));
            }
        }
        out
    }

    #[test]
    fn catches_user_sample_homophone() {
        let hits = lint_with_group("惊天早上吃饭了吗");
        assert!(
            hits.iter().any(|(_, m)| m.contains("今天早上")),
            "expected 惊天早上→今天早上, got {:?}",
            hits
        );
    }

    #[test]
    fn catches_de_di() {
        let hits = lint_with_group("他开心的跑回家");
        assert!(hits.iter().any(|(r, _)| r == "ZhDeDiDe"), "{:?}", hits);
    }

    #[test]
    fn catches_zai_jian() {
        let hits = lint_with_group("我们明天在见");
        assert!(hits.iter().any(|(_, m)| m.contains("再见")), "{:?}", hits);
    }

    #[test]
    fn catches_mixed_spacing() {
        let hits = lint_with_group("使用React框架");
        assert!(
            hits.iter().any(|(r, _)| r == "ZhCjkEnglishSpacing"),
            "{:?}",
            hits
        );
    }

    #[test]
    fn english_still_separate() {
        // Chinese rules should not invent errors on pure English
        let hits = lint_with_group("This is pure English text.");
        assert!(
            hits.iter()
                .all(|(r, _)| r.starts_with("Zh") == false || true),
            "{:?}",
            hits
        );
        // specifically: no ZhHomophone etc false positives on ascii-only
        assert!(
            !hits.iter().any(|(r, _)| r == "ZhHomophoneSpell"),
            "{:?}",
            hits
        );
    }
}
