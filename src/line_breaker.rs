extern crate unicode_width;

#[cfg(target_os = "macos")]
use crate::macos::*;
#[cfg(target_os = "windows")]
use crate::windows::*;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use crate::generic::*;

use crate::properties::*;

use core::char;
use core::str::CharIndices;
use unicode_width::UnicodeWidthChar;

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

fn get_linebreak_property_utf32_with_rule(codepoint: u32, rule: LineBreakRule, _ja_zh: bool) -> u8 {
    let codepoint = codepoint as usize;
    if codepoint < 0x20000 {
        if rule == LineBreakRule::Loose {
            let prop = UAX14_PROPERTY_TABLE[codepoint / 1024][(codepoint & 0x3ff)];
            return match prop {
                CJ => ID,
                _ => prop,
            };
        }

        if rule == LineBreakRule::Normal {
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
fn get_linebreak_property_with_rule(codepoint: char, rule: LineBreakRule, ja_zh: bool) -> u8 {
    get_linebreak_property_utf32_with_rule(codepoint as u32, rule, ja_zh)
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
    true
}

#[inline]
fn is_non_break_by_keepall(left: u8, right: u8) -> bool {
    (left == AI
        || left == AL
        || left == ID
        || left == NU
        || left == HY
        || left == H2
        || left == H3
        || left == CJ)
        && (right == AI
            || right == AL
            || right == ID
            || right == NU
            || right == HY
            || right == H2
            || right == H3
            || right == CJ)
}

#[inline]
fn get_break_state(left: u8, right: u8) -> i8 {
    UAX14_RULE_TABLE[((left as usize) - 1) * PROP_COUNT + (right as usize) - 1]
}

fn use_complex_breaking(codepoint: u32) -> bool {
    (codepoint >= 0xe00 && codepoint <= 0xe3a) || (codepoint >= 0xe40 && codepoint <= 0xe4e)
}

macro_rules! break_iterator_impl {
    ($name:ident, $iter_attr:ty, $char_type:ty) => {
        pub struct $name<'a> {
            iter: $iter_attr,
            current: Option<(usize, $char_type)>,
            break_rule: LineBreakRule,
            word_break_rule: WordBreakRule,
            ja_zh: bool,
        }

        impl<'a> Iterator for $name<'a> {
            type Item = usize;

            fn next(&mut self) -> Option<Self::Item> {
                if self.is_eof() {
                    return None;
                }

                loop {
                    let mut current_prop = self.get_linebreak_property();
                    // Handle LB25
                    // ( PR | PO) ? ( OP | HY ) ? NU (NU | SY | IS) * (CL | CP) ? ( PR | PO) ?
                    if self.word_break_rule != WordBreakRule::BreakAll
                        && self.word_break_rule != WordBreakRule::KeepAll
                        && (current_prop == PR
                            || current_prop == PO
                            || current_prop == OP_OP30
                            || current_prop == OP_EA
                            || current_prop == HY
                            || current_prop == NU)
                    {
                        let r = self.handle_lb25();
                        if r.is_some() && r.unwrap() > 0 {
                            // Most is EOF. if r == 0, this isn't LB25 rule.
                            return r;
                        }

                        // current_prop may be invalid, get it now.
                        current_prop = self.get_linebreak_property();
                    }

                    let next = self.iter.next();
                    if next.is_none() {
                        // EOF
                        let t = self.current.unwrap();
                        self.current = None;
                        return Some(t.0 + $name::char_len(t.1));
                    }
                    let left_codepoint = self.current;
                    self.current = next;
                    let right_prop = self.get_linebreak_property();

                    // CSS word-break property handling
                    if self.word_break_rule == WordBreakRule::BreakAll {
                        if current_prop == GL || right_prop == GL {
                            return Some(self.current.unwrap().0);
                        }
                        current_prop = match current_prop {
                            AL => ID,
                            NU => ID,
                            SA => ID,
                            _ => current_prop,
                        };
                    } else if self.word_break_rule == WordBreakRule::KeepAll {
                        if is_non_break_by_keepall(current_prop, right_prop) {
                            continue;
                        }
                    }

                    // CSS line-break property handling
                    if self.break_rule == LineBreakRule::Normal {
                        if self.is_break_by_normal() {
                            return Some(self.current.unwrap().0);
                        }
                    } else if self.break_rule == LineBreakRule::Loose {
                        if let Some(_breakable) = is_break_utf32_by_loose(
                            left_codepoint.unwrap().1 as u32,
                            self.current.unwrap().1 as u32,
                            current_prop,
                            right_prop,
                            self.ja_zh,
                        ) {
                            if _breakable {
                                return Some(self.current.unwrap().0);
                            }
                            continue;
                        }
                    } else if self.break_rule == LineBreakRule::Anywhere {
                        return Some(self.current.unwrap().0);
                    }

                    if current_prop == SA
                        && right_prop == SA
                        && use_complex_breaking(left_codepoint.unwrap().1 as u32)
                        && use_complex_breaking(self.current.unwrap().1 as u32)
                    {
                        let start_iter = self.iter.clone();
                        let start_point = self.current;
                        let mut s = Vec::new();
                        s.push(left_codepoint.unwrap().1 as u16);
                        s.push(self.current.unwrap().1 as u16);
                        loop {
                            self.current = self.iter.next();
                            if self.current.is_none() {
                                break;
                            }
                            if !use_complex_breaking(self.current.unwrap().1 as u32) {
                                break;
                            }
                            s.push(self.current.unwrap().1 as u16);
                        }
                        // Restore iterator to move to head of complex string
                        self.iter = start_iter;
                        self.current = start_point;
                        if let Some(first) = get_next_break_utf16(s.as_ptr(), s.len()) {
                            let mut i = 1;
                            loop {
                                if i == first {
                                    return Some(self.current.unwrap().0);
                                }
                                self.current = self.iter.next();
                                i += 1;
                            }
                        }
                    }

                    // If break_state is equals or grater than 0, it is alias of property.
                    let mut break_state = get_break_state(current_prop, right_prop);
                    if break_state >= 0 as i8 {
                        loop {
                            let prev = self.current.unwrap();
                            self.current = self.iter.next();
                            if self.current.is_none() {
                                // EOF
                                return Some(prev.0 + $name::char_len(prev.1));
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
                        return Some(self.current.unwrap().0);
                    }

                    if is_break(current_prop, right_prop) {
                        return Some(self.current.unwrap().0);
                    }
                }
            }
        }

        impl<'a> $name<'a> {
            #[inline]
            fn is_eof(&mut self) -> bool {
                if self.current.is_none() {
                    self.current = self.iter.next();
                    if self.current.is_none() {
                        return true;
                    }
                }
                return false;
            }

            fn handle_lb25(&mut self) -> Option<usize> {
                // Handle LB25
                // ( PR | PO) ? ( OP | HY ) ? NU (NU | SY | IS) * (CL | CP) ? ( PR | PO) ?
                let mut old_iter = self.iter.clone();
                let mut current = self.current;
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
                    self.current = None;
                    return Some(prev.unwrap().0 + $name::char_len(prev.unwrap().1));
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
                        self.current = None;
                        return Some(prev.unwrap().0 + $name::char_len(prev.unwrap().1));
                    }
                    state = self.get_linebreak_property_with_rule(current.unwrap().1);
                }

                if state == CL || state == CP {
                    old_iter = self.iter.clone();
                    prev = current;

                    current = self.iter.next();
                    if current.is_none() {
                        // EOF
                        self.current = None;
                        return Some(prev.unwrap().0 + $name::char_len(prev.unwrap().1));
                    }
                    state = self.get_linebreak_property_with_rule(current.unwrap().1);
                }

                if state == PR || state == PO {
                    self.current = current;
                    // Continue LB25 rule
                    return self.handle_lb25();
                }
                // Restore iterator that is NU/CL/CP position.
                self.iter = old_iter;
                self.current = prev;
                return None;
            }
        }
    };
}

break_iterator_impl!(LineBreakIterator, CharIndices<'a>, char);

impl<'a> LineBreakIterator<'a> {
    pub fn new(input: &str) -> LineBreakIterator {
        LineBreakIterator {
            iter: input.char_indices(),
            current: None,
            break_rule: LineBreakRule::Strict,
            word_break_rule: WordBreakRule::Normal,
            ja_zh: false,
        }
    }

    pub fn new_with_break_rule(
        input: &str,
        line_break_rule: LineBreakRule,
        word_break_rule: WordBreakRule,
        ja_zh: bool,
    ) -> LineBreakIterator {
        LineBreakIterator {
            iter: input.char_indices(),
            current: None,
            break_rule: line_break_rule,
            word_break_rule: word_break_rule,
            ja_zh: ja_zh,
        }
    }

    fn char_len(c: char) -> usize {
        c.len_utf8()
    }

    fn get_linebreak_property(&mut self) -> u8 {
        get_linebreak_property_with_rule(self.current.unwrap().1, self.break_rule, self.ja_zh)
    }

    fn get_linebreak_property_with_rule(&mut self, c: char) -> u8 {
        get_linebreak_property_utf32_with_rule(c as u32, self.break_rule, self.ja_zh)
    }

    fn is_break_by_normal(&mut self) -> bool {
        is_break_utf32_by_normal(self.current.unwrap().1 as u32, self.ja_zh)
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
    pub fn new(input: &[u8]) -> LineBreakIteratorLatin1 {
        LineBreakIteratorLatin1 {
            iter: Latin1Indices {
                front_offset: 0,
                iter: input,
            },
            current: None,
            break_rule: LineBreakRule::Strict,
            word_break_rule: WordBreakRule::Normal,
            ja_zh: false,
        }
    }

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
            current: None,
            break_rule: line_break_rule,
            word_break_rule: word_break_rule,
            ja_zh: ja_zh,
        }
    }

    fn char_len(_c: u8) -> usize {
        1
    }

    fn get_linebreak_property(&mut self) -> u8 {
        // No CJ on Latin1
        get_linebreak_property_latin1(self.current.unwrap().1)
    }

    fn get_linebreak_property_with_rule(&mut self, c: u8) -> u8 {
        // No CJ on Latin1
        get_linebreak_property_latin1(c)
    }

    fn is_break_by_normal(&mut self) -> bool {
        is_break_utf32_by_normal(self.current.unwrap().1 as u32, self.ja_zh)
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
    pub fn new(input: &[u16]) -> LineBreakIteratorUTF16 {
        LineBreakIteratorUTF16 {
            iter: UTF16Indices {
                front_offset: 0,
                iter: input,
            },
            current: None,
            break_rule: LineBreakRule::Strict,
            word_break_rule: WordBreakRule::Normal,
            ja_zh: false,
        }
    }

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
            current: None,
            break_rule: line_break_rule,
            word_break_rule: word_break_rule,
            ja_zh: ja_zh,
        }
    }

    fn char_len(c: u32) -> usize {
        if c > 0xffff {
            return 2;
        }
        1
    }

    fn get_linebreak_property(&mut self) -> u8 {
        get_linebreak_property_utf32_with_rule(self.current.unwrap().1, self.break_rule, self.ja_zh)
    }

    fn get_linebreak_property_with_rule(&mut self, c: u32) -> u8 {
        get_linebreak_property_utf32_with_rule(c, self.break_rule, self.ja_zh)
    }

    fn is_break_by_normal(&mut self) -> bool {
        is_break_utf32_by_normal(self.current.unwrap().1 as u32, self.ja_zh)
    }
}

#[cfg(test)]
mod tests {
    use crate::line_breaker::get_linebreak_property_with_rule;
    use crate::line_breaker::is_break;
    use crate::properties::*;
    use crate::LineBreakRule;

    fn get_linebreak_property(codepoint: char) -> u8 {
        get_linebreak_property_with_rule(codepoint, LineBreakRule::Strict, false)
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
