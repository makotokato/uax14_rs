# uax14_rs

A line breaker that is compatible with [Unicode Standard Annex #14][UAX14] and CSS properties.

[UAX14]: http://www.unicode.org/reports/tr14/

```rust
extern crate uax14_rs;
use uax14_rs::LineBreakIterator;

fn main ()
    let mut iter = LineBreakIterator("Hello World");
    let result: Vec<usize> = iter.map(|x| x).collect();
    println!("{}", result);
}
```
