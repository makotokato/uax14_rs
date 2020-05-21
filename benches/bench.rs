#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use test::Bencher;
    use uax14_rs::LineBreakIterator;
    use uax14_rs::LineBreakIteratorU8;

   #[bench]
   fn linebreak_iter(b: &mut Bencher) {
       // From about:mozilla
       let s = "The Beast adopted new raiment and studied the ways of Time and Space and Light and the Flow of energy through the Universe. From its studies, the Beast fashioned new structures from oxidised metal and proclaimed their glories. And the Beast’s followers rejoiced, finding renewed purpose in these teachings.";
       b.iter(|| LineBreakIterator::new(s).count())
   }

   #[bench]
   fn linebreak_iter_u8(b: &mut Bencher) {
       // From about:mozilla
       let s = "The Beast adopted new raiment and studied the ways of Time and Space and Light and the Flow of energy through the Universe. From its studies, the Beast fashioned new structures from oxidised metal and proclaimed their glories. And the Beast’s followers rejoiced, finding renewed purpose in these teachings.";
       b.iter(|| LineBreakIteratorU8::new(s.as_bytes()).count())
   }
}
