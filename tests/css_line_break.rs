extern crate uax14_rs;

use std::char;
use std::u32;
use uax14_rs::LineBreakIterator;
use uax14_rs::LineBreakIteratorLatin1;
use uax14_rs::LineBreakIteratorUTF16;
use uax14_rs::LineBreakRule;

#[test]
fn linebreak_strict() {
    // from css/css-text/line-break/line-break-*-011.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{3041}サ", LineBreakRule::Strict, false);
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-012.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{30FC}サ", LineBreakRule::Strict, false);
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-013.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{301C}サ", LineBreakRule::Strict, false);
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-014.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{3005}サ", LineBreakRule::Strict, false);
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-015a.xht
    // XXX ID x IN in UAX14. But why?
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{2025}\u{2025}サ", LineBreakRule::Strict, true);
    //assert_eq!(Some(3), iter.next());
    assert_eq!(Some(9), iter.next());
    assert_eq!(Some(12), iter.next());

    // from css/css-text/line-break/line-break-*-016a.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{30FB}サ", LineBreakRule::Strict, false);
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-017a.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{00B0}サ", LineBreakRule::Strict, false);
    assert_eq!(Some(5), iter.next());
    assert_eq!(Some(8), iter.next());

    // from css/css-text/line-break/line-break-*-018.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{20ac}サ", LineBreakRule::Strict, false);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(9), iter.next());
}

#[test]
fn linebreak_normal() {
    // from css/css-text/line-break/line-break-*-011.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{3041}サ", LineBreakRule::Normal, false);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-012.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{30FC}サ", LineBreakRule::Normal, false);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-loose-013.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{301C}サ", LineBreakRule::Normal, true);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-014.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{3005}サ", LineBreakRule::Normal, true);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-015.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{2025}\u{2025}サ", LineBreakRule::Normal, true);
    assert_eq!(Some(9), iter.next());
    assert_eq!(Some(12), iter.next());

    // from css/css-text/line-break/line-break-*-016a.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{30FB}サ", LineBreakRule::Normal, true);
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-017a.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{00B0}サ", LineBreakRule::Normal, true);
    assert_eq!(Some(5), iter.next());
    assert_eq!(Some(8), iter.next());

    // from css/css-text/line-break/line-break-*-018.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{20AC}サ", LineBreakRule::Normal, true);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(9), iter.next());
}

#[test]
fn linebreak_loose() {
    // from css/css-text/line-break/line-break-*-011.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{3041}サ", LineBreakRule::Loose, false);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-012.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{30FC}サ", LineBreakRule::Loose, false);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-loose-013.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{301C}サ", LineBreakRule::Loose, true);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-014.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{3005}サ", LineBreakRule::Loose, true);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-015.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{2025}\u{2025}サ", LineBreakRule::Loose, true);
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());
    assert_eq!(Some(12), iter.next());

    // from css/css-text/line-break/line-break-*-016a.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{30FB}サ", LineBreakRule::Loose, true);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());

    // from css/css-text/line-break/line-break-*-017a.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{00B0}サ", LineBreakRule::Loose, true);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(5), iter.next());
    assert_eq!(Some(8), iter.next());

    // from css/css-text/line-break/line-break-*-018.xht
    let mut iter =
        LineBreakIterator::new_with_break_rule("サ\u{20AC}サ", LineBreakRule::Loose, true);
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(6), iter.next());
    assert_eq!(Some(9), iter.next());
}
