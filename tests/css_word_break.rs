extern crate uax14_rs;

use uax14_rs::LineBreakIterator;
use uax14_rs::LineBreakRule;
use uax14_rs::WordBreakRule;

#[test]
fn wordbrak_breakall() {
    // from css/css-text/word-break/word-break-break-all-000.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "日本語",
        LineBreakRule::Strict,
        WordBreakRule::BreakAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/word-break/word-break-break-all-001.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "latin",
        LineBreakRule::Strict,
        WordBreakRule::BreakAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![1, 2, 3, 4, 5], result);

    // from css/css-text/word-break/word-break-break-all-002.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "한글이",
        LineBreakRule::Strict,
        WordBreakRule::BreakAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![3, 6, 9], result);

    // from css/css-text/word-break/word-break-break-all-004.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "التدويل نشاط التدويل",
        LineBreakRule::Strict,
        WordBreakRule::BreakAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![2, 4, 6, 8, 10, 12, 15, 17, 19, 21, 24, 26, 28, 30, 32, 34, 36, 38], result);

    // from css/css-text/word-break/word-break-break-all-008.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "हिन्दी हिन्दी हिन्दी",
        LineBreakRule::Strict,
        WordBreakRule::BreakAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 12, 19, 25, 31, 38, 44, 50, 56], result);
}

#[test]
fn wordbrak_keepall() {
    // from css/css-text/word-break/word-break-keep-all-000.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "latin",
        LineBreakRule::Strict,
        WordBreakRule::KeepAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![5], result);

    // from css/css-text/word-break/word-break-keep-all-001.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "日本語",
        LineBreakRule::Strict,
        WordBreakRule::KeepAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![9], result);

    // from css/css-text/word-break/word-break-keep-all-002.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "한글이",
        LineBreakRule::Strict,
        WordBreakRule::KeepAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![9], result);

    // from css/css-text/word-break/word-break-keep-all-003.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "และ",
        LineBreakRule::Strict,
        WordBreakRule::KeepAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![9], result);

    // from css/css-text/word-break/word-break-keep-all-005.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "字\u{3000}字",
        LineBreakRule::Strict,
        WordBreakRule::KeepAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 9], result);

    // from css/css-text/word-break/word-break-keep-all-006.html
    let mut iter = LineBreakIterator::new_with_break_rule(
        "字\u{3001}字",
        LineBreakRule::Strict,
        WordBreakRule::KeepAll,
        false,
    );
    let result: Vec<usize> = iter.map(|x| x).collect();
    assert_eq!(vec![6, 9], result);
}
