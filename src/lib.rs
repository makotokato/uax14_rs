mod properties;

use crate::properties::*;
use core::str::CharIndices;

fn get_linebreak_property(codepoint: char) -> u8 {
    let codepoint = codepoint as usize;
    if codepoint < 0x20000 {
        let table = UAX14_PROPERTY_TABLE[codepoint / 1024];
        return table[(codepoint & 0x3ff)];
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

fn get_linebreak_property_utf32(codepoint: u32) -> u8 {
    let codepoint = codepoint as usize;
    if codepoint < 0x20000 {
        let table = UAX14_PROPERTY_TABLE[codepoint / 1024];
        return table[(codepoint & 0x3ff)];
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

fn is_break(current: u8, next: u8) -> bool {
    let rule = UAX14_RULE_TABLE[((current as usize) - 1) * PROP_COUNT + (next as usize) - 1];
    if rule == -1 {
        return false;
    }
    true
}

fn get_break_state(current: u8, next: u8) -> i8 {
    UAX14_RULE_TABLE[((current as usize) - 1) * PROP_COUNT + (next as usize) - 1]
}

pub struct LineBreakIterator<'a> {
    iter: CharIndices<'a>,
    current: Option<(usize, char)>,
}

impl<'a> Iterator for LineBreakIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = self.iter.next();
            if self.current.is_none() {
                return None;
            }
        }

        let mut current_prop = get_linebreak_property(self.current.unwrap().1);
        loop {
            let next = self.iter.next();
            if next.is_none() {
                // EOF
                let t = self.current.unwrap();
                self.current = None;
                return Some(t.0 + t.1.len_utf8());
            }
            self.current = next;
            let next_prop = get_linebreak_property(self.current.unwrap().1);

            // Resolve state.
            let mut break_state = get_break_state(current_prop, next_prop);
            if break_state >= 0 as i8 {
                loop {
                    let it = self.iter.next();
                    if it.is_none() {
                        // EOF
                        let t = self.current.unwrap();
                        self.current = None;
                        return Some(t.0 + t.1.len_utf8());
                    }

                    self.current = it;
                    let prop = get_linebreak_property(it.unwrap().1);
                    break_state = get_break_state(break_state as u8, prop);
                    if break_state < 0 {
                        break;
                    }
                }
                if break_state == -1 {
                    current_prop = get_linebreak_property(self.current.unwrap().1);
                    continue;
                }
                return Some(self.current.unwrap().0);
            }
            if is_break(current_prop, next_prop) {
                return Some(self.current.unwrap().0);
            }
            current_prop = get_linebreak_property(self.current.unwrap().1);
        }
    }
}

impl<'a> LineBreakIterator<'a> {
    pub fn new(input: &str) -> LineBreakIterator {
        LineBreakIterator {
            iter: input.char_indices(),
            current: None,
        }
    }
}

pub struct LineBreakIteratorUTF16<'a> {
    iter: &'a [u16],
    current: usize,
}

impl<'a> Iterator for LineBreakIteratorUTF16<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.iter.len() {
            return None;
        }
        let mut current_point = self.iter[self.current] as u32;
        if (current_point & 0xfc00) == 0xd800 && (self.iter[self.current + 1] & 0xfc00) == 0xdc00 {
            current_point = ((current_point & 0x3ff) << 10)
                + ((self.iter[self.current + 1] as u32) & 0x3ff)
                + 0x10000;
            self.current = self.current + 1;
        }
        let mut current_prop = get_linebreak_property_utf32(current_point);
        loop {
            self.current = self.current + 1;
            if self.iter.len() > self.current {
                let mut next_point = self.iter[self.current] as u32;
                if (next_point & 0xfc00) == 0xd800
                    && (self.iter[self.current + 1] & 0xfc00) == 0xdc00
                {
                    next_point = ((next_point & 0x3ff) << 10)
                        + ((self.iter[self.current + 1] as u32) & 0x3ff)
                        + 0x10000;
                    self.current = self.current + 1;
                }
                let next_prop = get_linebreak_property_utf32(next_point);

                // Resolve state.
                let mut break_state = get_break_state(current_prop, next_prop);
                if break_state >= 0 {
                    loop {
                        self.current = self.current + 1;
                        if self.iter.len() > self.current {
                            let prop = get_linebreak_property_utf32(self.iter[self.current] as u32);
                            break_state = get_break_state(break_state as u8, prop);
                            if break_state < 0 {
                                break;
                            }
                        }
                    }
                    if break_state == -1 {
                        current_prop = get_linebreak_property_utf32(self.iter[self.current] as u32);
                        continue;
                    }
                    return Some(self.current);
                }

                if is_break(current_prop, next_prop) {
                    return Some(self.current);
                }
                current_prop = next_prop;
                continue;
            }
            return Some(self.current);
        }
    }
}

impl<'a> LineBreakIteratorUTF16<'a> {
    pub fn new(input: &[u16]) -> LineBreakIteratorUTF16 {
        LineBreakIteratorUTF16 {
            iter: input,
            current: 0,
        }
    }
}

pub struct LineBreakIteratorU8<'a> {
    iter: &'a [u8],
    current: usize,
}

impl<'a> Iterator for LineBreakIteratorU8<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.iter.len() {
            return None;
        }
        let mut current_prop = get_linebreak_property_utf32(self.iter[self.current] as u32);
        loop {
            self.current = self.current + 1;
            if self.iter.len() > self.current {
                let next_prop = get_linebreak_property_utf32(self.iter[self.current] as u32);

                // Resolve state.
                let mut break_state = get_break_state(current_prop, next_prop);
                if break_state >= 0 {
                    loop {
                        self.current = self.current + 1;
                        if self.iter.len() > self.current {
                            let prop = get_linebreak_property_utf32(self.iter[self.current] as u32);
                            break_state = get_break_state(break_state as u8, prop);
                            if break_state < 0 {
                                break;
                            }
                        }
                    }
                    if break_state == -1 {
                        current_prop = get_linebreak_property_utf32(self.iter[self.current] as u32);
                        continue;
                    }
                    return Some(self.current);
                }

                if is_break(current_prop, next_prop) {
                    return Some(self.current);
                }
                current_prop = next_prop;
                continue;
            }
            return Some(self.current);
        }
    }
}

impl<'a> LineBreakIteratorU8<'a> {
    pub fn new(input: &[u8]) -> LineBreakIteratorU8 {
        LineBreakIteratorU8 {
            iter: input,
            current: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_linebreak_property;
    use crate::is_break;
    use crate::properties::*;
    use crate::LineBreakIterator;
    use crate::LineBreakIteratorU8;
    use crate::LineBreakIteratorUTF16;

    #[test]
    fn linebreak_propery() {
        assert_eq!(get_linebreak_property('\u{0020}'), SP);
        assert_eq!(get_linebreak_property('\u{0022}'), QU);
        assert_eq!(get_linebreak_property('('), OP);
        assert_eq!(get_linebreak_property('\u{0030}'), NU);
        assert_eq!(get_linebreak_property('['), OP);
        assert_eq!(get_linebreak_property('\u{1f3fb}'), EM);
        assert_eq!(get_linebreak_property('\u{20000}'), ID);
        assert_eq!(get_linebreak_property('\u{e0020}'), CM);
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

    #[test]
    fn linebreak() {
        let mut iter = LineBreakIterator::new("hello world");
        assert_eq!(Some(6), iter.next());
        assert_eq!(Some(11), iter.next());

        iter = LineBreakIterator::new("$10 $10");
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(7), iter.next());

        // LB10

        // LB14
        iter = LineBreakIterator::new("[  abc def");
        assert_eq!(Some(7), iter.next());

        let input: [u8; 10] = [0x5B, 0x20, 0x20, 0x61, 0x62, 0x63, 0x20, 0x64, 0x65, 0x66];
        let mut iter_u8 = LineBreakIteratorU8::new(&input);
        assert_eq!(Some(7), iter_u8.next());

        let input: [u16; 10] = [0x5B, 0x20, 0x20, 0x61, 0x62, 0x63, 0x20, 0x64, 0x65, 0x66];
        let mut iter_u16 = LineBreakIteratorUTF16::new(&input);
        assert_eq!(Some(7), iter_u16.next());

        // LB15
        iter = LineBreakIterator::new("abc\u{0022}  (def");
        assert_eq!(Some(10), iter.next());

        let input: [u8; 10] = [0x61, 0x62, 0x63, 0x22, 0x20, 0x20, 0x28, 0x64, 0x65, 0x66];
        let mut iter_u8 = LineBreakIteratorU8::new(&input);
        assert_eq!(Some(10), iter_u8.next());

        let input: [u16; 10] = [0x61, 0x62, 0x63, 0x22, 0x20, 0x20, 0x28, 0x64, 0x65, 0x66];
        let mut iter_u16 = LineBreakIteratorUTF16::new(&input);
        assert_eq!(Some(10), iter_u16.next());

        // LB16
        iter = LineBreakIterator::new("\u{0029}\u{203C}");
        assert_eq!(Some(4), iter.next());
        iter = LineBreakIterator::new("\u{0029}  \u{203C}");
        assert_eq!(Some(6), iter.next());

        let input: [u16; 4] = [0x29, 0x20, 0x20, 0x203c];
        let mut iter_u16 = LineBreakIteratorUTF16::new(&input);
        assert_eq!(Some(4), iter_u16.next());

        // LB17
        iter = LineBreakIterator::new("\u{2014}\u{2014}aa");
        assert_eq!(Some(6), iter.next());

        iter = LineBreakIterator::new("\u{2014}\u{2014}  \u{2014}\u{2014}123 abc");
        assert_eq!(Some(14), iter.next());
        assert_eq!(Some(18), iter.next());
        assert_eq!(Some(21), iter.next());

        let input: [u16; 13] = [
            0x2014, 0x2014, 0x20, 0x20, 0x2014, 0x2014, 0x31, 0x32, 0x33, 0x20, 0x61, 0x62, 0x63,
        ];
        let mut iter_u16 = LineBreakIteratorUTF16::new(&input);
        assert_eq!(Some(6), iter_u16.next());

        iter = LineBreakIterator::new("\u{1F3FB} \u{1F3FB}");
        assert_eq!(Some(5), iter.next());
    }
}
