#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use test::Bencher;
    use uax14_rs::LineBreakIterator;
    use uax14_rs::LineBreakIteratorLatin1;
    use uax14_rs::LineBreakIteratorUTF16;

    const TEST_STR: &str = "The Beast adopted new raiment and studied the ways of Time and Space and Light and the Flow of energy through the Universe. From its studies, the Beast fashioned new structures from oxidised metal and proclaimed their glories. And the Beast’s followers rejoiced, finding renewed purpose in these teachings.";

    #[bench]
    fn linebreak_iter(b: &mut Bencher) {
        // From about:mozilla
        b.iter(|| LineBreakIterator::new(TEST_STR).count())
    }

    #[bench]
    fn linebreak_iter_latin1(b: &mut Bencher) {
        // From about:mozilla
        b.iter(|| LineBreakIteratorLatin1::new(TEST_STR.as_bytes()).count())
    }

    #[bench]
    fn linebreak_iter_utf16(b: &mut Bencher) {
        // From about:mozilla
        let mut utf16: [u16; 308] = [0; 308];
        TEST_STR
            .char_indices()
            .for_each(|(i, x)| utf16[i] = x as u16);
        b.iter(|| LineBreakIteratorUTF16::new(&utf16).count())
    }
}
