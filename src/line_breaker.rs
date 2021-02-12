extern crate unicode_width;

#[cfg(target_os = "android")]
use crate::android::*;
#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_os = "android"
)))]
use crate::generic::*;
#[cfg(target_os = "macos")]
use crate::macos::*;
#[cfg(target_os = "linux")]
use crate::pango::*;
#[cfg(target_os = "windows")]
use crate::windows::*;

use crate::lb_define::*;
use crate::properties::*;

use core::char;
use core::str::CharIndices;
use unicode_width::UnicodeWidthChar;

#[cfg(target_os = "android")]
use std::ffi::c_void;

#[derive(Copy, Clone, PartialEq)]
pub enum LineBreakRule {
    Normal,
    Strict,
    Loose,
    Anywhere,
}

#[derive(Copy, Clone, PartialEq)]
pub enum WordBreakRule {
    Normal,
    BreakAll,
    KeepAll,
}

fn get_linebreak_property_utf32_with_rule(
    codepoint: u32,
    line_break_rule: LineBreakRule,
    word_break_rule: WordBreakRule,
    _ja_zh: bool,
) -> u8 {
    let codepoint = codepoint as usize;
    if codepoint < 0x20000 {
        if word_break_rule == WordBreakRule::BreakAll {
            // Letter and number
            let prop = UAX14_PROPERTY_TABLE[codepoint / 1024][(codepoint & 0x3ff)];
            return match prop {
                CJ => ID, // All CJ's General category is Other_Letter (Lo).
                _ => prop,
            };
        }

        if line_break_rule == LineBreakRule::Loose {
            let prop = UAX14_PROPERTY_TABLE[codepoint / 1024][(codepoint & 0x3ff)];
            return match prop {
                CJ => ID,
                _ => prop,
            };
        }

        if line_break_rule == LineBreakRule::Normal {
            let prop = UAX14_PROPERTY_TABLE[codepoint / 1024][(codepoint & 0x3ff)];
            return match prop {
                CJ => ID,
                _ => prop,
            };
        }

        // CJ is mapped as NS on default
        return UAX14_PROPERTY_TABLE[codepoint / 1024][(codepoint & 0x3ff)];
    }

    match codepoint {
        0x20000..=0x2fffd => ID,
        0x30000..=0x3fffd => ID,
        0xe0001 => CM,
        0xe0020..=0xe007f => CM,
        0xe0100..=0xe01ef => CM,
        _ => XX,
    }
}

#[inline]
fn get_linebreak_property_latin1(codepoint: u8) -> u8 {
    let codepoint = codepoint as usize;
    UAX14_PROPERTY_TABLE[codepoint / 1024][(codepoint & 0x3ff)]
}

#[inline]
fn get_linebreak_property_with_rule(
    codepoint: char,
    linebreak_rule: LineBreakRule,
    wordbreak_rule: WordBreakRule,
    ja_zh: bool,
) -> u8 {
    get_linebreak_property_utf32_with_rule(codepoint as u32, linebreak_rule, wordbreak_rule, ja_zh)
}

#[inline]
fn is_break_utf32_by_normal(codepoint: u32, ja_zh: bool) -> bool {
    match codepoint as u32 {
        0x301C => ja_zh,
        0x30A0 => ja_zh,
        _ => false,
    }
}

#[inline]
fn is_break_utf32_by_loose(
    left_codepoint: u32,
    right_codepoint: u32,
    left_prop: u8,
    right_prop: u8,
    ja_zh: bool,
) -> Option<bool> {
    // breaks before hyphens
    if right_prop == BA {
        if left_prop == ID && (right_codepoint == 0x2010 || right_codepoint == 0x2013) {
            return Some(true);
        }
    } else if right_prop == NS {
        // breaks before certain CJK hyphen-like characters
        if right_codepoint == 0x301C || right_codepoint == 0x30A0 {
            return Some(ja_zh);
        }

        // breaks before iteration marks
        if right_codepoint == 0x3005
            || right_codepoint == 0x303B
            || right_codepoint == 0x309D
            || right_codepoint == 0x309E
            || right_codepoint == 0x30FD
            || right_codepoint == 0x30FE
        {
            return Some(true);
        }

        // breaks before certain centered punctuation marks:
        if right_codepoint == 0x30FB
            || right_codepoint == 0xFF1A
            || right_codepoint == 0xFF1B
            || right_codepoint == 0xFF65
            || right_codepoint == 0x203C
            || (right_codepoint >= 0x2047 && right_codepoint <= 0x2049)
        {
            return Some(ja_zh);
        }
    } else if right_prop == IN {
        // breaks between inseparable characters such as U+2025, U+2026 i.e. characters with the Unicode Line Break property IN
        return Some(true);
    } else if right_prop == EX {
        // breaks before certain centered punctuation marks:
        if right_codepoint == 0xFF01 || right_codepoint == 0xFF1F {
            return Some(ja_zh);
        }
    }

    // breaks before suffixes:
    // Characters with the Unicode Line Break property PO and the East Asian Width property
    if right_prop == PO
        && UnicodeWidthChar::width_cjk(char::from_u32(right_codepoint).unwrap()).unwrap() == 2
    {
        return Some(ja_zh);
    }
    // breaks after prefixes:
    // Characters with the Unicode Line Break property PR and the East Asian Width property
    if left_prop == PR
        && UnicodeWidthChar::width_cjk(char::from_u32(left_codepoint).unwrap()).unwrap() == 2
    {
        return Some(ja_zh);
    }
    None
}

#[inline]
fn is_break(left: u8, right: u8) -> bool {
    let rule = UAX14_RULE_TABLE[((left as usize) - 1) * PROP_COUNT + (right as usize) - 1];
    if rule == KEEP_RULE {
        return false;
    }
    if rule >= 0 {
        // need additional next characters to get break rule.
        return false;
    }
    true
}

#[inline]
fn is_non_break_by_keepall(left: u8, right: u8) -> bool {
    //  typographic letter units shouldn't be break
    (left == AI
        || left == AL
        || left == ID
        || left == NU
        || left == HY
        || left == H2
        || left == H3
        || left == JL
        || left == JV
        || left == JT
        || left == CJ)
        && (right == AI
            || right == AL
            || right == ID
            || right == NU
            || right == HY
            || right == H2
            || right == H3
            || right == JL
            || right == JV
            || right == JT
            || right == CJ)
}

#[inline]
fn get_break_state(left: u8, right: u8) -> i8 {
    UAX14_RULE_TABLE[((left as usize) - 1) * PROP_COUNT + (right as usize) - 1]
}

#[cfg(not(any(target_os = "macos", target_os = "android")))]
#[inline]
fn use_complex_breaking_utf32(codepoint: u32) -> bool {
    // Thai
    codepoint >= 0xe01 && codepoint <= 0xe7f
}
#[cfg(any(target_os = "macos", target_os = "android"))]
#[inline]
fn use_complex_breaking_utf32(codepoint: u32) -> bool {
    // Thai, Lao and Khmer
    (codepoint >= 0xe01 && codepoint <= 0xeff) || (codepoint >= 0x1780 && codepoint <= 0x17ff)
}

macro_rules! break_iterator_impl {
    ($name:ident, $iter_attr:ty, $char_type:ty) => {
        #[allow(dead_code)]
        pub struct $name<'a> {
            iter: $iter_attr,
            len: usize,
            current_pos_data: Option<(usize, $char_type)>,
            result_cache: Vec<usize>,
            break_rule: LineBreakRule,
            word_break_rule: WordBreakRule,
            ja_zh: bool,
            #[cfg(target_os = "android")]
            env: *mut c_void,
        }

        impl<'a> Iterator for $name<'a> {
            type Item = usize;

            fn next(&mut self) -> Option<Self::Item> {
                if self.is_eof() {
                    return None;
                }

                if !self.result_cache.is_empty() {
                    // We have break point cache by previous run.
                    let mut i = 0;
                    loop {
                        if i == *self.result_cache.first().unwrap() {
                            self.result_cache.remove(0);
                            self.result_cache = self.result_cache.iter().map(|r| r - i).collect();
                            return Some(self.current_pos_data.unwrap().0);
                        }
                        self.current_pos_data = self.iter.next();
                        if self.current_pos_data.is_none() {
                            // Reach EOF
                            self.result_cache.clear();
                            return Some(self.len);
                        }
                        i += 1;
                    }
                }

                loop {
                    let mut left_prop = self.get_linebreak_property();
                    // Handle LB25
                    // ( PR | PO) ? ( OP | HY ) ? NU (NU | SY | IS) * (CL | CP) ? ( PR | PO) ?
                    if self.word_break_rule != WordBreakRule::BreakAll
                        && self.word_break_rule != WordBreakRule::KeepAll
                        && (left_prop == PR
                            || left_prop == PO
                            || left_prop == OP_OP30
                            || left_prop == OP_EA
                            || left_prop == HY
                            || left_prop == NU)
                    {
                        let r = self.handle_lb25();
                        if r.is_some() && r.unwrap() > 0 {
                            // Most is EOF. if r == 0, this isn't LB25 rule.
                            return r;
                        }

                        // left_prop may be invalid, get it now.
                        left_prop = self.get_linebreak_property();
                    }

                    let left_codepoint = self.current_pos_data;
                    self.current_pos_data = self.iter.next();
                    if self.current_pos_data.is_none() {
                        // EOF
                        return Some(self.len);
                    }
                    let right_prop = self.get_linebreak_property();

                    // CSS word-break property handling
                    match self.word_break_rule {
                        WordBreakRule::BreakAll => {
                            left_prop = match left_prop {
                                AL => ID,
                                NU => ID,
                                SA => ID,
                                _ => left_prop,
                            };
                        }
                        WordBreakRule::KeepAll => {
                            if is_non_break_by_keepall(left_prop, right_prop) {
                                continue;
                            }
                        }
                        _ => (),
                    }

                    // CSS line-break property handling
                    match self.break_rule {
                        LineBreakRule::Normal => {
                            if self.is_break_by_normal() {
                                return Some(self.current_pos_data.unwrap().0);
                            }
                        }
                        LineBreakRule::Loose => {
                            if let Some(breakable) = is_break_utf32_by_loose(
                                left_codepoint.unwrap().1 as u32,
                                self.current_pos_data.unwrap().1 as u32,
                                left_prop,
                                right_prop,
                                self.ja_zh,
                            ) {
                                if breakable {
                                    return Some(self.current_pos_data.unwrap().0);
                                }
                                continue;
                            }
                        }
                        LineBreakRule::Anywhere => {
                            return Some(self.current_pos_data.unwrap().0);
                        }
                        _ => (),
                    };

                    // UAX14 doesn't have Thai etc, so use another way.
                    if $name::use_complex_breaking(left_codepoint.unwrap().1)
                        && $name::use_complex_breaking(self.current_pos_data.unwrap().1)
                    {
                        let result = self.handle_complex_language(left_codepoint.unwrap().1);
                        if result.is_some() {
                            return result;
                        }
                        // result is None means that platform API doesn't found any break opportunity.
                        // I may have to fetch text until non-SA character?.
                    }

                    // If break_state is equals or grater than 0, it is alias of property.
                    let mut break_state = get_break_state(left_prop, right_prop);
                    if break_state >= 0 as i8 {
                        loop {
                            self.current_pos_data = self.iter.next();
                            if self.current_pos_data.is_none() {
                                // EOF
                                return Some(self.len);
                            }

                            let prop = self.get_linebreak_property();
                            break_state = get_break_state(break_state as u8, prop);
                            if break_state < 0 {
                                break;
                            }
                        }
                        if break_state == KEEP_RULE {
                            continue;
                        }
                        return Some(self.current_pos_data.unwrap().0);
                    }

                    if is_break(left_prop, right_prop) {
                        return Some(self.current_pos_data.unwrap().0);
                    }
                }
            }
        }

        impl<'a> $name<'a> {
            #[inline]
            fn is_eof(&mut self) -> bool {
                if self.current_pos_data.is_none() {
                    self.current_pos_data = self.iter.next();
                    if self.current_pos_data.is_none() {
                        return true;
                    }
                }
                return false;
            }

            fn handle_lb25(&mut self) -> Option<usize> {
                // Handle LB25
                // ( PR | PO) ? ( OP | HY ) ? NU (NU | SY | IS) * (CL | CP) ? ( PR | PO) ?
                let mut old_iter = self.iter.clone();
                let mut current = self.current_pos_data;
                let mut state = self.get_linebreak_property();

                if state == PR || state == PO {
                    let left = current;
                    let left_prop = state;

                    current = self.iter.next();
                    if current.is_some() {
                        state = self.get_linebreak_property_with_rule(current.unwrap().1);

                        // If left is PR, it might apply loose rule.
                        if self.break_rule == LineBreakRule::Loose {
                            if let Some(_breakable) = is_break_utf32_by_loose(
                                left.unwrap().1 as u32,
                                current.unwrap().1 as u32,
                                left_prop,
                                state,
                                self.ja_zh,
                            ) {
                                // reset state since this cannot apply LB25.
                                state = 0;
                            }
                        }
                    }
                    // If reaching EOF, restore iterator
                }

                if state == OP_OP30 || state == HY || state == OP_EA {
                    current = self.iter.next();
                    if current.is_some() {
                        state = self.get_linebreak_property_with_rule(current.unwrap().1);
                    }
                    // If reaching EOF, restore iterator
                }

                if current.is_none() || state != NU {
                    // Not match for LB25 due to EOF. Restore before parsing.
                    self.iter = old_iter;
                    return Some(0);
                }

                // This should apply LB25 rule.

                old_iter = self.iter.clone();
                let mut prev = current;

                current = self.iter.next();
                if current.is_none() {
                    // EOF
                    self.current_pos_data = None;
                    return Some(self.len);
                }

                state = self.get_linebreak_property_with_rule(current.unwrap().1);
                loop {
                    if state != NU && state != SY && state != IS {
                        break;
                    }
                    old_iter = self.iter.clone();
                    prev = current;
                    current = self.iter.next();
                    if current.is_none() {
                        // EOF
                        self.current_pos_data = None;
                        return Some(self.len);
                    }
                    state = self.get_linebreak_property_with_rule(current.unwrap().1);
                }

                if state == CL || state == CP {
                    old_iter = self.iter.clone();
                    prev = current;

                    current = self.iter.next();
                    if current.is_none() {
                        // EOF
                        self.current_pos_data = None;
                        return Some(self.len);
                    }
                    state = self.get_linebreak_property_with_rule(current.unwrap().1);
                }

                if state == PR || state == PO {
                    self.current_pos_data = current;
                    // Continue LB25 rule
                    return self.handle_lb25();
                }
                // Restore iterator that is NU/CL/CP position.
                self.iter = old_iter;
                self.current_pos_data = prev;
                return None;
            }

            // UAX14 doesn't define line break rules for some languages such as Thai.
            // These languages uses dictionary-based breaker, so we use OS's line breaker instead.
            fn handle_complex_language(&mut self, left_codepoint: $char_type) -> Option<usize> {
                let start_iter = self.iter.clone();
                let start_point = self.current_pos_data;
                let mut s = Vec::new();
                s.push(left_codepoint as u16);
                loop {
                    s.push(self.current_pos_data.unwrap().1 as u16);
                    self.current_pos_data = self.iter.next();
                    if self.current_pos_data.is_none() {
                        break;
                    }
                    if !$name::use_complex_breaking(self.current_pos_data.unwrap().1) {
                        break;
                    }
                }
                // Restore iterator to move to head of complex string
                self.iter = start_iter;
                self.current_pos_data = start_point;
                let breaks = self.get_line_break_utf16(s.as_ptr(), s.len())?;
                let mut i = 1;
                self.result_cache = breaks;
                // result_cache vector is utf-16 index that is in BMP.
                loop {
                    if i == *self.result_cache.first().unwrap() {
                        self.result_cache.remove(0);
                        self.result_cache = self.result_cache.iter().map(|r| r - i).collect();
                        return Some(self.current_pos_data.unwrap().0);
                    }
                    self.current_pos_data = self.iter.next();
                    if self.current_pos_data.is_none() {
                        self.result_cache.clear();
                        return Some(self.len);
                    }
                    i += 1;
                }
            }
        }
    };
}

break_iterator_impl!(LineBreakIterator, CharIndices<'a>, char);

impl<'a> LineBreakIterator<'a> {
    /// Create line break iterator
    pub fn new(input: &str) -> LineBreakIterator {
        LineBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            break_rule: LineBreakRule::Strict,
            word_break_rule: WordBreakRule::Normal,
            ja_zh: false,
            #[cfg(target_os = "android")]
            env: std::ptr::null_mut(),
        }
    }

    /// Create line break iterator with CSS rules
    pub fn new_with_break_rule(
        input: &str,
        line_break_rule: LineBreakRule,
        word_break_rule: WordBreakRule,
        ja_zh: bool,
    ) -> LineBreakIterator {
        LineBreakIterator {
            iter: input.char_indices(),
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            break_rule: line_break_rule,
            word_break_rule: word_break_rule,
            ja_zh: ja_zh,
            #[cfg(target_os = "android")]
            env: std::ptr::null_mut(),
        }
    }

    fn get_linebreak_property(&mut self) -> u8 {
        get_linebreak_property_with_rule(
            self.current_pos_data.unwrap().1,
            self.break_rule,
            self.word_break_rule,
            self.ja_zh,
        )
    }

    fn get_linebreak_property_with_rule(&mut self, c: char) -> u8 {
        get_linebreak_property_utf32_with_rule(
            c as u32,
            self.break_rule,
            self.word_break_rule,
            self.ja_zh,
        )
    }

    fn is_break_by_normal(&mut self) -> bool {
        is_break_utf32_by_normal(self.current_pos_data.unwrap().1 as u32, self.ja_zh)
    }

    #[inline]
    fn use_complex_breaking(c: char) -> bool {
        use_complex_breaking_utf32(c as u32)
    }

    #[cfg(target_os = "android")]
    fn get_line_break_utf16(&mut self, text: *const u16, length: usize) -> Option<Vec<usize>> {
        if self.env.is_null() {
            return None;
        }
        get_line_break_utf16(self.env, text, length)
    }

    #[cfg(not(target_os = "android"))]
    fn get_line_break_utf16(&mut self, text: *const u16, length: usize) -> Option<Vec<usize>> {
        get_line_break_utf16(text, length)
    }

    /// Set JNI env for Android
    #[cfg(target_os = "android")]
    pub fn set_jni_env(&mut self, env: *mut c_void) {
        self.env = env;
    }
}

// Latin1 version of line break iterator for FFI

#[derive(Clone)]
struct Latin1Indices<'a> {
    front_offset: usize,
    iter: &'a [u8],
}

impl<'a> Iterator for Latin1Indices<'a> {
    type Item = (usize, u8);

    #[inline]
    fn next(&mut self) -> Option<(usize, u8)> {
        if self.front_offset >= self.iter.len() {
            return None;
        }
        let ch = self.iter[self.front_offset];
        let index = self.front_offset;
        self.front_offset += 1;
        Some((index, ch))
    }
}

break_iterator_impl!(LineBreakIteratorLatin1, Latin1Indices<'a>, u8);

impl<'a> LineBreakIteratorLatin1<'a> {
    /// Create line break iterator
    pub fn new(input: &[u8]) -> LineBreakIteratorLatin1 {
        LineBreakIteratorLatin1 {
            iter: Latin1Indices {
                front_offset: 0,
                iter: input,
            },
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            break_rule: LineBreakRule::Strict,
            word_break_rule: WordBreakRule::Normal,
            ja_zh: false,
            #[cfg(target_os = "android")]
            env: std::ptr::null_mut(),
        }
    }

    /// Create line break iterator with CSS rules
    pub fn new_with_break_rule(
        input: &[u8],
        line_break_rule: LineBreakRule,
        word_break_rule: WordBreakRule,
        ja_zh: bool,
    ) -> LineBreakIteratorLatin1 {
        LineBreakIteratorLatin1 {
            iter: Latin1Indices {
                front_offset: 0,
                iter: input,
            },
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            break_rule: line_break_rule,
            word_break_rule: word_break_rule,
            ja_zh: ja_zh,
            #[cfg(target_os = "android")]
            env: std::ptr::null_mut(),
        }
    }

    fn get_linebreak_property(&mut self) -> u8 {
        // No CJ on Latin1
        get_linebreak_property_latin1(self.current_pos_data.unwrap().1)
    }

    fn get_linebreak_property_with_rule(&mut self, c: u8) -> u8 {
        // No CJ on Latin1
        get_linebreak_property_latin1(c)
    }

    fn is_break_by_normal(&mut self) -> bool {
        is_break_utf32_by_normal(self.current_pos_data.unwrap().1 as u32, self.ja_zh)
    }

    #[inline]
    fn use_complex_breaking(_c: u8) -> bool {
        false
    }

    fn get_line_break_utf16(&mut self, _text: *const u16, _length: usize) -> Option<Vec<usize>> {
        None
    }
}

// UTF16

#[derive(Clone)]
struct UTF16Indices<'a> {
    front_offset: usize,
    iter: &'a [u16],
}

impl<'a> Iterator for UTF16Indices<'a> {
    type Item = (usize, u32);

    #[inline]
    fn next(&mut self) -> Option<(usize, u32)> {
        if self.front_offset >= self.iter.len() {
            return None;
        }
        let ch = self.iter[self.front_offset];
        let index = self.front_offset;
        self.front_offset += 1;

        if (ch & 0xfc00) != 0xd800 {
            return Some((index, ch as u32));
        }

        let mut ch = ch as u32;
        if self.front_offset < self.iter.len() {
            let next = self.iter[self.front_offset] as u32;
            if (next & 0xfc00) == 0xdc00 {
                ch = ((ch & 0x3ff) << 10) + (next & 0x3ff) + 0x10000;
                self.front_offset += 1;
                return Some((index, ch));
            }
        }
        Some((index, ch))
    }
}

break_iterator_impl!(LineBreakIteratorUTF16, UTF16Indices<'a>, u32);

impl<'a> LineBreakIteratorUTF16<'a> {
    /// Create line break iterator
    pub fn new(input: &[u16]) -> LineBreakIteratorUTF16 {
        LineBreakIteratorUTF16 {
            iter: UTF16Indices {
                front_offset: 0,
                iter: input,
            },
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            break_rule: LineBreakRule::Strict,
            word_break_rule: WordBreakRule::Normal,
            ja_zh: false,
            #[cfg(target_os = "android")]
            env: std::ptr::null_mut(),
        }
    }

    /// Create line break iterator with CSS rules
    pub fn new_with_break_rule(
        input: &[u16],
        line_break_rule: LineBreakRule,
        word_break_rule: WordBreakRule,
        ja_zh: bool,
    ) -> LineBreakIteratorUTF16 {
        LineBreakIteratorUTF16 {
            iter: UTF16Indices {
                front_offset: 0,
                iter: input,
            },
            len: input.len(),
            current_pos_data: None,
            result_cache: Vec::new(),
            break_rule: line_break_rule,
            word_break_rule: word_break_rule,
            ja_zh: ja_zh,
            #[cfg(target_os = "android")]
            env: std::ptr::null_mut(),
        }
    }

    fn get_linebreak_property(&mut self) -> u8 {
        get_linebreak_property_utf32_with_rule(
            self.current_pos_data.unwrap().1,
            self.break_rule,
            self.word_break_rule,
            self.ja_zh,
        )
    }

    fn get_linebreak_property_with_rule(&mut self, c: u32) -> u8 {
        get_linebreak_property_utf32_with_rule(c, self.break_rule, self.word_break_rule, self.ja_zh)
    }

    fn is_break_by_normal(&mut self) -> bool {
        is_break_utf32_by_normal(self.current_pos_data.unwrap().1 as u32, self.ja_zh)
    }

    #[inline]
    fn use_complex_breaking(c: u32) -> bool {
        use_complex_breaking_utf32(c)
    }

    #[cfg(target_os = "android")]
    fn get_line_break_utf16(&mut self, text: *const u16, length: usize) -> Option<Vec<usize>> {
        if self.env.is_null() {
            return None;
        }
        get_line_break_utf16(self.env, text, length)
    }

    #[cfg(not(target_os = "android"))]
    fn get_line_break_utf16(&mut self, text: *const u16, length: usize) -> Option<Vec<usize>> {
        get_line_break_utf16(text, length)
    }

    /// Set JNI env for Android
    #[cfg(target_os = "android")]
    pub fn set_jni_env(&mut self, env: *mut c_void) {
        self.env = env;
    }
}

#[cfg(test)]
mod tests {
    use crate::lb_define::*;
    use crate::line_breaker::get_linebreak_property_with_rule;
    use crate::line_breaker::is_break;
    use crate::LineBreakRule;
    use crate::WordBreakRule;

    fn get_linebreak_property(codepoint: char) -> u8 {
        get_linebreak_property_with_rule(
            codepoint,
            LineBreakRule::Strict,
            WordBreakRule::Normal,
            false,
        )
    }

    #[test]
    fn linebreak_propery() {
        assert_eq!(get_linebreak_property('\u{0020}'), SP);
        assert_eq!(get_linebreak_property('\u{0022}'), QU);
        assert_eq!(get_linebreak_property('('), OP_OP30);
        assert_eq!(get_linebreak_property('\u{0030}'), NU);
        assert_eq!(get_linebreak_property('['), OP_OP30);
        assert_eq!(get_linebreak_property('\u{1f3fb}'), EM);
        assert_eq!(get_linebreak_property('\u{20000}'), ID);
        assert_eq!(get_linebreak_property('\u{e0020}'), CM);
        assert_eq!(get_linebreak_property('\u{3041}'), CJ);
        assert_eq!(get_linebreak_property('\u{0025}'), PO);
        assert_eq!(get_linebreak_property('\u{00A7}'), AI);
        assert_eq!(get_linebreak_property('\u{50005}'), XX);
        assert_eq!(get_linebreak_property('\u{17D6}'), NS);
        assert_eq!(get_linebreak_property('\u{2014}'), B2);
    }

    #[test]
    fn break_rule() {
        // LB4
        assert_eq!(is_break(BK, AL), true);
        // LB5
        assert_eq!(is_break(CR, LF), false);
        assert_eq!(is_break(CR, AL), true);
        assert_eq!(is_break(LF, AL), true);
        assert_eq!(is_break(NL, AL), true);
        // LB6
        assert_eq!(is_break(AL, BK), false);
        assert_eq!(is_break(AL, CR), false);
        assert_eq!(is_break(AL, LF), false);
        assert_eq!(is_break(AL, NL), false);
        // LB7
        assert_eq!(is_break(AL, SP), false);
        assert_eq!(is_break(AL, ZW), false);
        // LB8
        // LB8a
        assert_eq!(is_break(ZWJ, AL), false);
        // LB9
        assert_eq!(is_break(AL, ZWJ), false);
        assert_eq!(is_break(AL, CM), false);
        assert_eq!(is_break(ID, ZWJ), false);
        // LB10
        assert_eq!(is_break(ZWJ, SP), false);
        assert_eq!(is_break(SP, CM), true);
        // LB11
        assert_eq!(is_break(AL, WJ), false);
        assert_eq!(is_break(WJ, AL), false);
        // LB12
        assert_eq!(is_break(GL, AL), false);
        // LB12a
        assert_eq!(is_break(AL, GL), false);
        assert_eq!(is_break(SP, GL), true);
        // LB13
        assert_eq!(is_break(AL, CL), false);
        assert_eq!(is_break(AL, CP), false);
        assert_eq!(is_break(AL, EX), false);
        assert_eq!(is_break(AL, IS), false);
        assert_eq!(is_break(AL, SY), false);
        // LB18
        assert_eq!(is_break(SP, AL), true);
        // LB19
        assert_eq!(is_break(AL, QU), false);
        assert_eq!(is_break(QU, AL), false);
        // LB20
        assert_eq!(is_break(AL, CB), true);
        assert_eq!(is_break(CB, AL), true);
        // LB20
        assert_eq!(is_break(AL, BA), false);
        assert_eq!(is_break(AL, HY), false);
        assert_eq!(is_break(AL, NS), false);
        assert_eq!(is_break(BB, AL), false);
        // LB21
        assert_eq!(is_break(AL, BA), false);
        assert_eq!(is_break(ID, BA), false);
        assert_eq!(is_break(ID, NS), false);
        // LB21a
        // LB21b
        assert_eq!(is_break(SY, HL), false);
        // LB22
        assert_eq!(is_break(AL, IN), false);
        // LB 23
        assert_eq!(is_break(AL, NU), false);
        assert_eq!(is_break(HL, NU), false);
        // LB 23a
        assert_eq!(is_break(PR, ID), false);
        assert_eq!(is_break(PR, EB), false);
        assert_eq!(is_break(PR, EM), false);
        assert_eq!(is_break(ID, PO), false);
        assert_eq!(is_break(EB, PO), false);
        assert_eq!(is_break(EM, PO), false);
        // LB26
        assert_eq!(is_break(JL, JL), false);
        assert_eq!(is_break(JL, JV), false);
        assert_eq!(is_break(JL, H2), false);
        // LB27
        assert_eq!(is_break(JL, IN), false);
        assert_eq!(is_break(JL, PO), false);
        assert_eq!(is_break(PR, JL), false);
        // LB28
        assert_eq!(is_break(AL, AL), false);
        assert_eq!(is_break(HL, AL), false);
        // LB29
        assert_eq!(is_break(IS, AL), false);
        assert_eq!(is_break(IS, HL), false);
        // LB30b
        assert_eq!(is_break(EB, EM), false);
        // LB31
        assert_eq!(is_break(ID, ID), true);
    }
}
