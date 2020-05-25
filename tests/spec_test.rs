extern crate uax14_rs;

use std::char;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::u32;
use uax14_rs::LineBreakIterator;

#[test]
fn run_line_break_test() {
    let failed = [
        "\u{0023}\u{2329}",
        "\u{0023}\u{0308}\u{2329}",
        "\u{0023}\u{00A7}",
        "\u{0023}\u{0308}\u{00A7}",
        "\u{0023}\u{50005}",
        "\u{0023}\u{0308}\u{50005}",
        "\u{0023}\u{0E01}",
        "\u{0023}\u{0308}\u{0E01}",
        "\u{0023}\u{3041}",
        "\u{0023}\u{0308}\u{3041}",
        "\u{2014}\u{0308}\u{0020}\u{00A0}",
        "\u{2014}\u{0020}\u{00A0}",
        "\u{2014}\u{0020}\u{0001}",
        "\u{2014}\u{0308}\u{0020}\u{0001}",
        "\u{2014}\u{0020}\u{200D}",
        "\u{2014}\u{0308}\u{0020}\u{200D}",
        "\u{2014}\u{3041}",
        "\u{2014}\u{0308}\u{3041}",
        "\u{0009}\u{3041}",
        "\u{0009}\u{0308}\u{3041}",
        "\u{007D}\u{0020}\u{00A0}",
        "\u{007D}\u{0308}\u{0020}\u{00A0}",
        "\u{007D}\u{0025}",
        "\u{007D}\u{0308}\u{0025}",
        "\u{007D}\u{0024}",
        "\u{007D}\u{0308}\u{0024}",
        "\u{007D}\u{0020}\u{0001}",
        "\u{007D}\u{0308}\u{0020}\u{0001}",
        "\u{007D}\u{0020}\u{200D}",
        "\u{007D}\u{0308}\u{0020}\u{200D}",
        "\u{007D}\u{3041}",
        "\u{007D}\u{0020}\u{3041}",
        "\u{007D}\u{0308}\u{3041}",
        "\u{007D}\u{0308}\u{0020}\u{3041}",
        "\u{0021}\u{3041}",
        "\u{0021}\u{0308}\u{3041}",
        "\u{AC00}\u{3041}",
        "\u{AC00}\u{0308}\u{3041}",
        "\u{AC01}\u{3041}",
        "\u{AC01}\u{0308}\u{3041}",
    ];

    let f = File::open("tools/LineBreakTest.txt");
    let f = BufReader::new(f.unwrap());
    for line in f.lines() {
        let line = line.unwrap();
        if line.starts_with("#") {
            continue;
        }
        let mut r = line.split("#");
        let r = r.next();
        let v: Vec<_> = r.unwrap().split_ascii_whitespace().collect();
        let mut b: Vec<_> = Vec::new();
        let mut c: Vec<_> = Vec::new();
        let mut count = 0;
        let mut char_len = 0;
        loop {
            if count >= v.len() {
                break;
            }
            if count % 2 == 1 {
                let ch = char::from_u32(u32::from_str_radix(v[count], 16).unwrap()).unwrap();
                c.push(ch);
                char_len = char_len + ch.len_utf8();
            } else {
                if v[count] == "\u{00d7}" {
                } else {
                    assert_eq!(v[count], "\u{00f7}");
                    b.push(char_len);
                }
            }
            count = count + 1
        }
        let s: String = c.into_iter().collect();
        let mut iter = LineBreakIterator::new(&s);
        if failed.contains(&&s.as_str()) {
            assert_ne!(iter.next(), Some(b[0]), "{}", line);
            continue;
        }
        assert_eq!(iter.next(), Some(b[0]), "{}", line);
    }
}
