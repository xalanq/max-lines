//! The `max_lines` crate can read many lines once a time from BufRead and
//! **without any extra memory allocation.**.
//!
//! Have fun!
//!
//! # Quick Start
//!
//! ```norun
//! extern crate max_lines;
//! use max_lines::*;
//!
//! let file = File::open(path).unwrap();
//! let buf = BufReader::with_capacity(1024 * 1024 * 32, file);
//! buf.max_lines(1000).for_each(|slice| {
//!     // now slice's type is
//!     // std::vec::Vec<std::result::Result<std::string::String, std::io::Error>>
//! });
//! ```
//!
//! you can also read a single line:
//!
//! ```norun
//! extern crate max_lines;
//! use max_lines::*;
//!
//! let file = File::open(path).unwrap();
//! let buf = BufReader::with_capacity(1024 * 1024 * 32, file);
//! let mut iter = buf.max_lines(1000);
//! let s = iter.single(); // std::result::Result<std::string::String, std::io::Error>
//! buf.max_lines(1000).for_each(|slice| {
//!     // now slice's type is
//!     // std::vec::Vec<std::result::Result<std::string::String, std::io::Error>>
//! });
//! ```

use std::io::{BufRead, BufReader, Read, Result};

#[derive(Debug)]
pub struct MaxLines<B> {
    buf: B,
    max_lines: usize,
}

impl<B: BufRead> MaxLines<B> {
    /// Read a single line.
    ///
    /// # Example
    ///
    /// ```norun
    /// extern crate max_lines;
    /// use max_lines::*;
    ///
    /// let file = File::open(path).unwrap();
    /// let buf = BufReader::with_capacity(1024 * 1024 * 32, file);
    /// let mut iter = buf.max_lines(1000);
    /// let s = iter.single(); // std::result::Result<std::string::String, std::io::Error>
    /// buf.max_lines(1000).for_each(|slice| {
    ///     // now slice's type is
    ///     // std::vec::Vec<std::result::Result<std::string::String, std::io::Error>>
    /// });
    pub fn single(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with("\n") {
                    buf.pop();
                    if buf.ends_with("\r") {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

impl<B: BufRead> Iterator for MaxLines<B> {
    type Item = Vec<Result<String>>;

    /// # Example
    /// extern crate max_lines;
    /// use max_lines::*;
    ///
    /// ```norun
    /// let file = File::open(path).unwrap();
    /// let buf = BufReader::with_capacity(1024 * 1024 * 32, file);
    /// buf.max_lines(1000).for_each(|slice| {
    ///     // now slice's type is
    ///     // std::vec::Vec<std::result::Result<std::string::String, std::io::Error>>
    /// });
    fn next(&mut self) -> Option<Vec<Result<String>>> {
        let mut ret = Vec::new();
        let mut i = 0;
        while i < self.max_lines {
            match self.single() {
                None => break,
                Some(s) => ret.push(s),
            }
            i += 1;
        }
        if ret.len() == 0 {
            None
        } else {
            Some(ret)
        }
    }
}

pub trait MaxLinesIterator: Sized {
    /// max_lines is the maximum number of lines MaxLines can read.
    fn max_lines(self, max_lines: usize) -> MaxLines<Self>
    where
        Self: Sized,
    {
        MaxLines {
            buf: self,
            max_lines,
        }
    }
}

impl<R: Read> MaxLinesIterator for BufReader<R> {}
