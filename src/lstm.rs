use core::char::decode_utf16;
use core::str::Chars;
use icu_segmenter_lstm::lstm::Lstm;

// TODO:
// json file is big, So I should use anoher binary format like npy.
// But provided npy uses tensorflow dtype.
const THAI_MODEL: &[u8; 373466] =
    include_bytes!("../data/Thai_codepoints_exclusive_model4_heavy/weights.json");
const BURMESE_MODEL: &[u8; 475209] =
    include_bytes!("../data/Burmese_codepoints_exclusive_model4_heavy/weights.json");

lazy_static! {
    static ref THAI_LSTM: Lstm = {
        let lstm_data = serde_json::from_slice(THAI_MODEL).expect("JSON syntax error");
        Lstm::try_new(lstm_data).unwrap()
    };
    static ref BURMESE_LSTM: Lstm = {
        let lstm_data = serde_json::from_slice(BURMESE_MODEL).expect("JSON syntax error");
        Lstm::try_new(lstm_data).unwrap()
    };
}

// LSTM model depends on language, So we have to switch models per lang.

#[derive(PartialEq)]
enum Language {
    BURMESE,
    THAI,
    UNKNOWN,
}

fn get_language(codepoint: u32) -> Language {
    match codepoint as u32 {
        0xe01..=0xe7f => Language::THAI,
        0x1000..=0x109f => Language::BURMESE,
        _ => Language::UNKNOWN,
    }
}

struct LanguageIterator<'a> {
    input: Chars<'a>,
    last: Option<char>,
}

impl<'a> LanguageIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
            last: None,
        }
    }
}

impl<'a> Iterator for LanguageIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = "".to_string();

        if self.last.is_none() {
            self.last = self.input.next();
        }
        if self.last.is_none() {
            return None;
        }
        let lang = get_language(self.last.unwrap() as u32);
        s.push(self.last.unwrap());
        loop {
            let c = self.input.next();
            if c.is_none() {
                self.last = None;
                break;
            }
            self.last = c;
            let new_lang = get_language(c.unwrap() as u32);
            if lang != new_lang {
                break;
            }
            s.push(c.unwrap());
        }
        Some(s)
    }
}

// A word break iterator using LSTM model. Input string have to be same language.

struct LstmSegmenterIterator {
    input: String,
    bies_str: String,
    pos: usize,
    pos_utf8: usize,
}

impl Iterator for LstmSegmenterIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            self.pos_utf8 += self.input.chars().nth(self.pos).unwrap().len_utf8();
            self.pos += 1;
            if ch == 'e' && self.bies_str.len() > self.pos {
                return Some(self.pos_utf8);
            }
        }
    }
}

impl LstmSegmenterIterator {
    #[cfg(test)]
    pub fn new(input: &str) -> Self {
        let lang = get_language(input.chars().next().unwrap() as u32);
        let lstm = match lang {
            Language::THAI => &*THAI_LSTM,
            Language::BURMESE => &*BURMESE_LSTM,
            _ => panic!("Unsupported"),
        };
        let lstm_output = lstm.word_segmenter(input);
        Self {
            input: input.to_string(),
            bies_str: lstm_output,
            pos: 0,
            pos_utf8: 0,
        }
    }
}

struct LstmSegmenterIteratorUTF16 {
    bies_str: String,
    pos: usize,
}

impl Iterator for LstmSegmenterIteratorUTF16 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            // This ch is always in bitmap.
            self.pos += 1;
            if ch == 'e' && self.bies_str.len() > self.pos {
                return Some(self.pos);
            }
        }
    }
}

impl LstmSegmenterIteratorUTF16 {
    pub fn new(input: &str) -> Self {
        let lang = get_language(input.chars().next().unwrap() as u32);
        let lstm = match lang {
            Language::THAI => &*THAI_LSTM,
            Language::BURMESE => &*BURMESE_LSTM,
            _ => panic!("Unsupported"),
        };
        let lstm_output = lstm.word_segmenter(input);
        Self {
            bies_str: lstm_output,
            pos: 0,
        }
    }
}

#[cfg(test)]
pub fn get_line_break_utf8(input: &str) -> Option<Vec<usize>> {
    let mut result: Vec<usize> = Vec::new();
    let mut lang_iter = LanguageIterator::new(&input);
    let mut offset = 0;
    loop {
        let str_per_lang = lang_iter.next();
        if str_per_lang.is_none() {
            break;
        }
        if offset != 0 {
            result.push(offset);
        }
        let lstm_iter = LstmSegmenterIterator::new(str_per_lang.as_ref().unwrap());
        let mut r: Vec<usize> = lstm_iter.map(|n| offset + n).collect();
        result.append(&mut r);
        offset += str_per_lang.unwrap().len();
    }
    if result.is_empty() {
        return None;
    }
    Some(result)
}

pub fn get_line_break_utf16(input: &[u16]) -> Option<Vec<usize>> {
    let s: String = decode_utf16(input.iter().cloned())
        .map(|r| r.unwrap())
        .collect();
    let mut result: Vec<usize> = Vec::new();
    let mut lang_iter = LanguageIterator::new(&s);
    let mut offset = 0;
    loop {
        let str_per_lang = lang_iter.next();
        if str_per_lang.is_none() {
            break;
        }
        if offset != 0 {
            result.push(offset);
        }
        let lstm_iter = LstmSegmenterIteratorUTF16::new(str_per_lang.as_ref().unwrap());
        let mut r: Vec<usize> = lstm_iter.map(|n| offset + n).collect();
        result.append(&mut r);
        offset += str_per_lang
            .as_ref()
            .unwrap()
            .chars()
            .fold(0, |n, c| n + c.len_utf16());
    }
    if result.is_empty() {
        return None;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::lstm::get_line_break_utf16;
    use crate::lstm::get_line_break_utf8;
    use crate::lstm::LanguageIterator;
    use crate::lstm::LstmSegmenterIterator;

    #[test]
    fn language_boundary() {
        const TEST_STR_THAI: &str = "ภาษาไทยภาษาไทย";
        const TEST_STR_BURMESE: &str = "ဗမာနွယ်ဘာသာစကားများ";

        let mut iter = LanguageIterator::new(TEST_STR_THAI);

        assert_eq!(iter.next(), Some(TEST_STR_THAI.to_string()), "Thai");
        assert_eq!(iter.next(), None, "EOF");

        let mut sample = String::from(TEST_STR_THAI);
        sample.push_str(TEST_STR_BURMESE);
        let mut iter = LanguageIterator::new(&sample);
        assert_eq!(iter.next(), Some(TEST_STR_THAI.to_string()), "Thai");
        assert_eq!(iter.next(), Some(TEST_STR_BURMESE.to_string()), "Burmese");
        assert_eq!(iter.next(), None, "EOF");
    }

    #[test]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";

        let breaks = get_line_break_utf8(TEST_STR);
        assert_eq!(breaks.unwrap(), [12, 21, 33], "Thai test");

        let mut iter = LstmSegmenterIterator::new(TEST_STR);
        assert_eq!(iter.next(), Some(12), "1st Thai word");
        assert_eq!(iter.next(), Some(21), "2nd Thai word");
        assert_eq!(iter.next(), Some(33), "3rd Thai word");
    }

    #[test]
    fn thai_word_break_utf16() {
        let text: [u16; 14] = [
            0x0e20, 0x0e32, 0x0e29, 0x0e32, 0x0e44, 0x0e17, 0x0e22, 0x0e20, 0x0e32, 0x0e29, 0x0e32,
            0x0e44, 0x0e17, 0x0e22,
        ];
        let breaks = get_line_break_utf16(&text);
        assert_eq!(breaks.unwrap(), [4, 7, 11], "Thai test using UTF-16");

        let text: [u16; 4] = [0x0e20, 0x0e32, 0x0e29, 0x0e32];
        let breaks = get_line_break_utf16(&text);
        assert_eq!(breaks, None, "Thai test");
    }

    #[test]
    fn burmese_word_break() {
        const TEST_STR: &str = "မြန်မာဘာသာစကား";

        let breaks = get_line_break_utf8(TEST_STR);
        assert_eq!(breaks.unwrap(), [12, 18, 30], "Burmese test");

        let mut iter = LstmSegmenterIterator::new(TEST_STR);
        assert_eq!(iter.next(), Some(12), "1st Burmese word");
        assert_eq!(iter.next(), Some(18), "2nd Burmese word");
        assert_eq!(iter.next(), Some(30), "3rd Burmese word");
    }

    #[test]
    fn combined_word_break() {
        const TEST_STR_THAI: &str = "ภาษาไทยภาษาไทย";
        const TEST_STR_BURMESE: &str = "ဗမာနွယ်ဘာသာစကားများ";
        let mut sample = String::from(TEST_STR_THAI);
        sample.push_str(TEST_STR_BURMESE);

        let breaks = get_line_break_utf8(&sample);
        assert_eq!(
            breaks.unwrap(),
            [12, 21, 33, 42, 51, 63, 75, 87],
            "Combined test"
        );
    }
}
