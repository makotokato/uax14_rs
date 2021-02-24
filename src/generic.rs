use icu_segmenter_lstm::lstm::Lstm;
use std::char::decode_utf16;

const THAI_MODEL: &[u8; 373466] =
    include_bytes!("../data/Thai_codepoints_exclusive_model4_heavy/weights.json");

lazy_static! {
    static ref THAI_LSTM: Lstm = {
        let lstm_data = serde_json::from_slice(THAI_MODEL).expect("JSON syntax error");
        Lstm::try_new(lstm_data).unwrap()
    };
}

struct LstmSegmenterIterator {
    bies_str: String,
    pos: usize,
}

impl Iterator for LstmSegmenterIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            self.pos += 1;
            if ch == 'e' && self.bies_str.len() > self.pos {
                return Some(self.pos);
            }
        }
    }
}

impl LstmSegmenterIterator {
    pub fn new(lstm: &Lstm, input: &str) -> Self {
        let lstm_output = lstm.word_segmenter(input);
        Self {
            bies_str: lstm_output,
            pos: 0,
        }
    }
}

pub fn get_line_break_utf16(text: *const u16, length: usize) -> Option<Vec<usize>> {
    let slice = unsafe { std::slice::from_raw_parts(text, length) };
    let s: String = decode_utf16(slice.iter().cloned())
        .map(|r| r.unwrap())
        .collect();
    let mut iter = LstmSegmenterIterator::new(&*THAI_LSTM, &s);
    let result: Vec<usize> = iter.map(|i| i).collect();
    if result.is_empty() {
        return None;
    }
    Some(result)
}

mod tests {
    use crate::generic::get_line_break_utf16;

    #[test]
    fn macos_line_break() {
        let text: [u16; 14] = [
            0x0e20, 0x0e32, 0x0e29, 0x0e32, 0x0e44, 0x0e17, 0x0e22, 0x0e20, 0x0e32, 0x0e29, 0x0e32,
            0x0e44, 0x0e17, 0x0e22,
        ];
        let breaks = get_line_break_utf16(text.as_ptr(), text.len());
        assert_eq!(breaks.unwrap(), [4, 7, 11], "Thai test");

        let text: [u16; 4] = [0x0e20, 0x0e32, 0x0e29, 0x0e32];
        let breaks = get_line_break_utf16(text.as_ptr(), text.len());
        assert_eq!(breaks, None, "Thai test");
    }
}
