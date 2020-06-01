extern crate uax14_rs;

use std::char;
use std::u32;
use uax14_rs::LineBreakIterator;
use uax14_rs::LineBreakIteratorLatin1;
use uax14_rs::LineBreakIteratorUTF16;
use uax14_rs::LineBreakRule;
use uax14_rs::WordBreakRule;

fn strict(s: &str, ja_zh: bool, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let mut iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().map(|x| x).collect();
    let mut iter = LineBreakIteratorUTF16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

fn normal(s: &str, ja_zh: bool, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let mut iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().map(|x| x).collect();
    let mut iter = LineBreakIteratorUTF16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

fn loose(s: &str, ja_zh: bool, expect_utf8: Vec<usize>, expect_utf16: Vec<usize>) {
    let mut iter = LineBreakIterator::new_with_break_rule(
        s,
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(expect_utf8, result, "{}", s);

    let s_utf16: Vec<u16> = s.encode_utf16().map(|x| x).collect();
    let mut iter = LineBreakIteratorUTF16::new_with_break_rule(
        &s_utf16,
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        ja_zh,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(expect_utf16, result, "{}", s);
}

#[test]
fn linebreak_strict() {
    // from css/css-text/line-break/line-break-*-011.xht
    strict("サ\u{3041}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-012.xht
    strict("サ\u{30FC}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-013.xht
    strict("サ\u{301C}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-014.xht
    strict("サ\u{3005}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-015a.xht
    // XXX ID x IN in UAX14. But why?
    strict("サ\u{2025}\u{2025}サ", false, vec![9, 12], vec![3, 4]);

    // from css/css-text/line-break/line-break-*-016a.xht
    strict("サ\u{30FB}サ", false, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-017a.xht
    strict("サ\u{00B0}サ", false, vec![5, 8], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-018.xht
    strict("サ\u{20AC}サ", false, vec![3, 9], vec![1, 3]);
}

#[test]
fn linebreak_normal() {
    // from css/css-text/line-break/line-break-*-011.xht
    normal("サ\u{3041}サ", false, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-012.xht
    normal("サ\u{30FC}サ", false, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-013.xht
    normal("サ\u{301C}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-014.xht
    normal("サ\u{3005}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-015.xht
    normal("サ\u{2025}\u{2025}サ", true, vec![9, 12], vec![3, 4]);

    // from css/css-text/line-break/line-break-*-016a.xht
    normal("サ\u{30FB}サ", true, vec![6, 9], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-017a.xht
    normal("サ\u{00B0}サ", true, vec![5, 8], vec![2, 3]);

    // from css/css-text/line-break/line-break-*-018.xht
    normal("サ\u{20AC}サ", true, vec![3, 9], vec![1, 3]);
}

#[test]
fn linebreak_loose() {
    // from css/css-text/line-break/line-break-*-011.xht
    loose("サ\u{3041}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-012.xht
    loose("サ\u{30FC}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-loose-013.xht
    loose("サ\u{301C}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-014.xht
    loose("サ\u{3005}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-015.xht
    loose("サ\u{2025}\u{2025}サ", true, vec![6, 9, 12], vec![2, 3, 4]);

    // from css/css-text/line-break/line-break-*-016a.xht
    loose("サ\u{30FB}サ", true, vec![3, 6, 9], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-017a.xht
    loose("サ\u{00B0}サ", true, vec![3, 5, 8], vec![1, 2, 3]);

    // from css/css-text/line-break/line-break-*-018.xht
    loose("サ\u{20AC}サ", true, vec![3, 6, 9], vec![1, 2, 3]);
}
