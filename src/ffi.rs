use crate::LineBreakIteratorLatin1;
use crate::LineBreakIteratorUTF16;
use crate::LineBreakRule;
use crate::WordBreakRule;

/// Process line break using C-like FFI
#[no_mangle]
pub extern "C" fn line_breaker_utf16(
    source: *const u16,
    length: usize,
    word_break_rule: u32,
    line_break_rule: u32,
    ja_zh: bool,
    break_output: *mut u8,
) {
    unsafe { std::intrinsics::write_bytes(break_output, 0, length) };

    let str_buffer = unsafe { std::slice::from_raw_parts(source, length) };
    let break_buffer = unsafe { std::slice::from_raw_parts_mut(break_output, length) };

    // sync https://searchfox.org/mozilla-central/source/intl/lwbrk/LineBreaker.h
    let word_break = match word_break_rule {
        1 => WordBreakRule::BreakAll,
        2 => WordBreakRule::KeepAll,
        _ => WordBreakRule::Normal,
    };

    // sync https://searchfox.org/mozilla-central/source/intl/lwbrk/LineBreaker.h
    let line_break = match line_break_rule {
        1 => LineBreakRule::Loose,
        2 => LineBreakRule::Normal,
        4 => LineBreakRule::Anywhere,
        _ => LineBreakRule::Strict, //strict and auto
    };

    LineBreakIteratorUTF16::new_with_break_rule(str_buffer.as_ref(), line_break, word_break, ja_zh)
        .for_each(|i| break_buffer[i] = 1);
}

/// Process line break using C-like FFI
#[no_mangle]
pub extern "C" fn line_breaker_latin1(
    source: *const u8,
    length: usize,
    word_break_rule: u32,
    line_break_rule: u32,
    ja_zh: bool,
    break_output: *mut u8,
) {
    unsafe { core::intrinsics::write_bytes(break_output, 0, length) };

    let str_buffer = unsafe { core::slice::from_raw_parts(source, length) };
    let break_buffer = unsafe { core::slice::from_raw_parts_mut(break_output, length) };

    // sync https://searchfox.org/mozilla-central/source/intl/lwbrk/LineBreaker.h
    let word_break = match word_break_rule {
        1 => WordBreakRule::BreakAll,
        2 => WordBreakRule::KeepAll,
        _ => WordBreakRule::Normal,
    };

    // sync https://searchfox.org/mozilla-central/source/intl/lwbrk/LineBreaker.h
    let line_break = match line_break_rule {
        1 => LineBreakRule::Loose,
        2 => LineBreakRule::Normal,
        3 => LineBreakRule::Strict, // auto
        4 => LineBreakRule::Anywhere,
        _ => LineBreakRule::Strict,
    };

    LineBreakIteratorLatin1::new_with_break_rule(
        str_buffer.as_ref(),
        line_break,
        word_break,
        ja_zh,
    )
    .for_each(|i| break_buffer[i] = 1);
}
