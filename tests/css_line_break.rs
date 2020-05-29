extern crate uax14_rs;

use std::char;
use std::u32;
use uax14_rs::LineBreakIterator;
use uax14_rs::LineBreakIteratorLatin1;
use uax14_rs::LineBreakIteratorUTF16;
use uax14_rs::LineBreakRule;
use uax14_rs::WordBreakRule;

#[test]
fn linebreak_strict() {
    // from css/css-text/line-break/line-break-*-011.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{3041}サ",
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 9], result);

    // from css/css-text/line-break/line-break-*-012.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{30FC}サ",
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 9], result);

    // from css/css-text/line-break/line-break-*-013.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{301C}サ",
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 9], result);

    // from css/css-text/line-break/line-break-*-014.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{3005}サ",
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 9], result);

    // from css/css-text/line-break/line-break-*-015a.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{2025}\u{2025}サ",
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    // XXX ID x IN in UAX14. But why?
    assert_eq!(vec![9, 12], result);

    // from css/css-text/line-break/line-break-*-016a.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{30FB}サ",
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 9], result);

    // from css/css-text/line-break/line-break-*-017a.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{00B0}サ",
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![5, 8], result);

    // from css/css-text/line-break/line-break-*-018.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{20ac}サ",
        LineBreakRule::Strict,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 9], result);
}

#[test]
fn linebreak_normal() {
    // from css/css-text/line-break/line-break-*-011.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{3041}サ",
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/line-break/line-break-*-012.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{30FC}サ",
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/line-break/line-break-loose-013.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{301C}サ",
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/line-break/line-break-*-014.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{3005}サ",
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/line-break/line-break-*-015.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{2025}\u{2025}サ",
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![9, 12], result);

    // from css/css-text/line-break/line-break-*-016a.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{30FB}サ",
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 9], result);

    // from css/css-text/line-break/line-break-*-017a.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{00B0}サ",
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![5, 8], result);

    // from css/css-text/line-break/line-break-*-018.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{20AC}サ",
        LineBreakRule::Normal,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 9], result);
}

#[test]
fn linebreak_loose() {
    // from css/css-text/line-break/line-break-*-011.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{3041}サ",
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/line-break/line-break-*-012.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{30FC}サ",
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/line-break/line-break-loose-013.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{301C}サ",
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/line-break/line-break-*-014.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{3005}サ",
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/line-break/line-break-*-015.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{2025}\u{2025}サ",
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 9, 12], result);

    // from css/css-text/line-break/line-break-*-016a.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{30FB}サ",
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/line-break/line-break-*-017a.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{00B0}サ",
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 5, 8], result);

    // from css/css-text/line-break/line-break-*-018.xht
    let mut iter = LineBreakIterator::new_with_break_rule(
        "サ\u{20AC}サ",
        LineBreakRule::Loose,
        WordBreakRule::Normal,
        true,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);
}
