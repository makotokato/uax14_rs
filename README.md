![](https://github.com/makotokato/uax14_rs/workflows/CI/badge.svg)

# uax14_rs

A line breaker that is compatible with [Unicode Standard Annex #14][UAX14] and CSS properties.

[UAX14]: http://www.unicode.org/reports/tr14/

```rust
extern crate uax14_rs;
use uax14_rs::LineBreakIterator;

fn main () {
    let mut iter = LineBreakIterator::new("Hello World");
    let result: Vec<usize> = iter.map(|x| x).collect();
    println!("{}", result);
}
```

With CSS property.
```rust
extern crate uax14_rs;
use uax14_rs::LineBreakIterator;

fn main () {
    let mut iter =
        LineBreakIterator::new_with_break_rule("Hello World", LineBreakRule::Strict, WordBreakRule::BreakAll, false);
    let result: Vec<usize> = iter.map(|x| x).collect();
    println!("{}", result);
}
```

Use Latin 1 string for C binding and etc.

```rust
extern crate uax14_rs;
use uax14_rs::LineBreakIteratorLatin1;

fn main () {
    let s = "Hello World";
    let mut iter = LineBreakIteratorLatin1::new(s.as_bytes());
    let result: Vec<usize> = iter.map(|x| x).collect();
    println!("{}", result);
}
```

If using Android API (24+) for Thai,
```rust
{
    let s = "Hello World";
    let mut iter = LineBreakIteratorLatin1::new(s.as_bytes());
    iter.set_jni_env(env);
    let result: Vec<usize> = iter.map(|x| x).collect();
    println!("{}", result);
}
```
See android-examples.

## Generating property table

Copy the following files to tools directory. Then run `generate_properties.py` in `tools` directory.
- <https://www.unicode.org/Public/UCD/latest/ucd/LineBreak.txt>
- <https://www.unicode.org/Public/UCD/latest/ucd/EastAsianWidth.txt>

## Run cargo test

Download LineBreakTest.txt from <https://www.unicode.org/Public/UCD/latest/ucd/auxiliary/LineBreakTest.txt>, then copy it to tools directory.

## TODO

Migrate <https://github.com/unicode-org/lstm_word_segmentation> instead of platform API.
