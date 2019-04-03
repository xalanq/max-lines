# Max Lines

The `max_lines` crate can read many lines once a time from BufRead and **without any extra memory allocation**.

Have fun!

# Quick Start

```rust
extern crate max_lines;
use max_lines::*;

let file = File::open(path).unwrap();
let buf = BufReader::with_capacity(1024 * 1024 * 32, file);
buf.max_lines(1000).for_each(|slice| {
    // now slice's type is
    // std::vec::Vec<std::result::Result<std::string::String, std::io::Error>>
});
```

you can also read a single line:

```rust
extern crate max_lines;
use max_lines::*;

let file = File::open(path).unwrap();
let buf = BufReader::with_capacity(1024 * 1024 * 32, file);
let mut iter = buf.max_lines(1000);
let s = iter.single(); // std::result::Result<std::string::String, std::io::Error>
buf.max_lines(1000).for_each(|slice| {
    // now slice's type is
    // std::vec::Vec<std::result::Result<std::string::String, std::io::Error>>
});
```
