#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use test::Bencher;
    use uax14_rs::LineBreakIterator;
    use uax14_rs::LineBreakIteratorLatin1;
    use uax14_rs::LineBreakIteratorUTF16;
    use uax14_rs::LineBreakRule;
    use uax14_rs::WordBreakRule;

    // Example is MIT license.
    const TEST_STR: &str = "Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions: The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.";
    const TEST_STR2: &str = "ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย ภาษาไทยภาษาไทย";

    #[bench]
    fn linebreak_iter(b: &mut Bencher) {
        b.iter(|| LineBreakIterator::new(TEST_STR).count())
    }

    #[bench]
    fn linebreak_iter_latin1(b: &mut Bencher) {
        b.iter(|| LineBreakIteratorLatin1::new(TEST_STR.as_bytes()).count())
    }

    #[bench]
    fn linebreak_iter_utf16(b: &mut Bencher) {
        let utf16: Vec<u16> = TEST_STR.encode_utf16().map(|x| x).collect();
        b.iter(|| LineBreakIteratorUTF16::new(&utf16).count())
    }

    #[bench]
    fn linebreak_iter_latin1_with_css_rule(b: &mut Bencher) {
        b.iter(|| {
            LineBreakIteratorLatin1::new_with_break_rule(
                TEST_STR.as_bytes(),
                LineBreakRule::Anywhere,
                WordBreakRule::BreakAll,
                true,
            )
            .count()
        })
    }

    #[bench]
    fn linebreak_iter_utf16_with_css_rule(b: &mut Bencher) {
        // From about:mozilla
        let utf16: Vec<u16> = TEST_STR.encode_utf16().map(|x| x).collect();
        b.iter(|| {
            LineBreakIteratorUTF16::new_with_break_rule(
                &utf16,
                LineBreakRule::Anywhere,
                WordBreakRule::BreakAll,
                true,
            )
            .count()
        })
    }

    #[bench]
    fn linebreak_iter_utf16_th(b: &mut Bencher) {
        let utf16: Vec<u16> = TEST_STR2.encode_utf16().map(|x| x).collect();
        b.iter(|| LineBreakIteratorUTF16::new(&utf16).count())
    }
}
