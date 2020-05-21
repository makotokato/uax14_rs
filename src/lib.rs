mod properties;

use crate::properties::*;
use core::str::CharIndices;

pub fn get_linebreak_property(codepoint: char) -> u8 {
    let codepoint = codepoint as usize;
    if codepoint < 0x30000 {
        let table = UAX14_PROPERTY_TABLE[codepoint / 1024];
        return table[(codepoint & 0x3ff)];
    }
    XX
}

pub fn get_linebreak_property_utf32(codepoint: u32) -> u8 {
    let codepoint = codepoint as usize;
    if codepoint < 0x30000 {
        let table = UAX14_PROPERTY_TABLE[codepoint / 1024];
        return table[(codepoint & 0x3ff)];
    }
    XX
}

fn is_break(current: u8, next: u8) -> bool {
    let rule = UAX14_RULE_TABLE[((current as usize) - 1) * PROP_COUNT + (next as usize) - 1];
    if rule == -1 {
        return false;
    }
    true
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
            if let Some(next) = self.iter.next() {
                let next_prop = get_linebreak_property(next.1);
                // LB14
                if current_prop == OP {
                    if next_prop != SP {
                        self.current = Some(next);
                        current_prop = next_prop;
                        continue;
                    }
                    // Skip SP
                    self.skip_space_only();
                    current_prop = get_linebreak_property(self.current.unwrap().1);
                    if current_prop != SP {
                        continue;
                    }

                    // Reach EOF
                    let index = self.current.unwrap().0;
                    self.current = None;
                    return Some(index);
                }

                // LB15
                if current_prop == QU {
                    if next_prop == OP {
                        self.current = Some(next);
                        current_prop = next_prop;
                        continue;
                    }

                    if next_prop == SP {
                        if let Some(r) = self.skip_space(OP) {
                            self.current = Some(r);
                            current_prop = get_linebreak_property(self.current.unwrap().1);
                            continue;
                        }
                    }
                }
                // LB16
                if current_prop == CL || current_prop == CP {
                    if next_prop == NS {
                        self.current = Some(next);
                        current_prop = next_prop;
                        continue;
                    }

                    if next_prop == SP {
                        if let Some(r) = self.skip_space(NS) {
                            self.current = Some(r);
                            current_prop = get_linebreak_property(self.current.unwrap().1);
                            continue;
                        }
                    }
                }

                // LB17
                if current_prop == B2 {
                    if next_prop == B2 {
                        self.current = Some(next);
                        current_prop = next_prop;
                        continue;
                    }

                    if next_prop == SP {
                        if let Some(r) = self.skip_space(B2) {
                            self.current = Some(r);
                            current_prop = get_linebreak_property(self.current.unwrap().1);
                            continue;
                        }
                    }
                    self.current = Some(next);
                    return Some(next.0);
                }

                // LB21a
                if current_prop == HL && (next_prop == HY || next_prop == BA) {
                    // Fetch next
                    self.current = self.iter.next();
                    if self.current.is_none() {
                        return Some(next.0);
                    }
                    current_prop = get_linebreak_property(self.current.unwrap().1);
                    continue;
                }

                if is_break(current_prop, next_prop) {
                    self.current = Some(next);
                    return Some(next.0);
                }
                self.current = Some(next);
                current_prop = next_prop
            } else {
                let t = self.current.unwrap();
                self.current = None;
                return Some(t.0 + t.1.len_utf8());
            }
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

    fn skip_space_only(&mut self) {
        loop {
            let next = self.iter.next();
            if next.is_none() {
                return;
            }
            self.current = next;
            if get_linebreak_property(next.unwrap().1) != SP {
                return;
            }
        }
    }

    fn skip_space(&mut self, prop: u8) -> Option<(usize, char)> {
        let backup_iter = self.iter.clone();
        let mut last_index;
        loop {
            last_index = self.iter.next();
            if last_index.is_some() {
                if get_linebreak_property(last_index.unwrap().1) == SP {
                    continue;
                }
            } else {
                self.iter = backup_iter;
                return None;
            }
            break;
        }
        if get_linebreak_property(last_index.unwrap().1) == prop {
            // Found SP* x [prop]
            return last_index;
        }
        // Restore
        self.iter = backup_iter;
        return None;
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
                // LB14
                if current_prop == OP {
                    if next_prop != SP {
                        current_prop = next_prop;
                        continue;
                    }
                    loop {
                        self.current = self.current + 1;
                        if self.iter.len() > self.current {
                            if get_linebreak_property_utf32(self.iter[self.current] as u32) == SP {
                                continue;
                            }
                            break;
                        }
                        return Some(self.current - 1);
                    }
                    continue;
                }

                // LB15
                if current_prop == QU {
                    if next_prop == OP {
                        current_prop = next_prop;
                        continue;
                    }

                    if next_prop == SP {
                        let backup_current = self.current;
                        loop {
                            self.current = self.current + 1;
                            if self.iter.len() > self.current {
                                if get_linebreak_property_utf32(self.iter[self.current] as u32)
                                    == SP
                                {
                                    continue;
                                }
                                break;
                            }
                            return Some(self.current - 1);
                        }
                        if get_linebreak_property_utf32(self.iter[self.current] as u32) == OP {
                            continue;
                        }
                        self.current = backup_current;
                    }
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

    #[test]
    fn linebreak_propery() {
        assert_eq!(get_linebreak_property('\u{0020}'), SP);
        assert_eq!(get_linebreak_property('\u{0022}'), QU);
        assert_eq!(get_linebreak_property('('), OP);
        assert_eq!(get_linebreak_property('\u{0030}'), NU);
        assert_eq!(get_linebreak_property('['), OP);
        assert_eq!(get_linebreak_property('\u{1f3fb}'), EM);
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

        // LB14
        iter = LineBreakIterator::new("[  abc def");
        assert_eq!(Some(7), iter.next());

        let input: [u8; 10] = [0x5B, 0x20, 0x20, 0x61, 0x62, 0x63, 0x20, 0x64, 0x65, 0x66];
        let mut iter_u8 = LineBreakIteratorU8::new(&input);
        assert_eq!(Some(7), iter_u8.next());

        // LB15
        iter = LineBreakIterator::new("abc\u{0022}  (def");
        assert_eq!(Some(10), iter.next());

        let input: [u8; 10] = [0x61, 0x62, 0x63, 0x22, 0x20, 0x20, 0x28, 0x64, 0x65, 0x66];
        iter_u8 = LineBreakIteratorU8::new(&input);
        assert_eq!(Some(10), iter_u8.next());

        // LB16
        iter = LineBreakIterator::new("\u{0029}\u{203C}");
        assert_eq!(Some(4), iter.next());
        iter = LineBreakIterator::new("\u{0029}  \u{203C}");
        assert_eq!(Some(6), iter.next());

        // LB17
        iter = LineBreakIterator::new("\u{2014}\u{2014}aa");
        assert_eq!(Some(6), iter.next());

        iter = LineBreakIterator::new("\u{2014}\u{2014}  \u{2014}\u{2014}123 abc");
        assert_eq!(Some(14), iter.next());
        assert_eq!(Some(18), iter.next());
        assert_eq!(Some(21), iter.next());

        //iter = LineBreakIterator::new("\u{3001}\u{548C}");
        iter = LineBreakIterator::new("\u{1F3FB} \u{1F3FB}");
        assert_eq!(Some(5), iter.next());
    }
}
