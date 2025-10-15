// webrust/src/text/string.rs
//! # Python-like String Methods
//!
//! Comprehensive Python-inspired string manipulation with smart pattern
//! recognition and zero-cost abstractions (all methods are `#[inline]`).
//!
//! ## The `splitby()` Revolution
//!
//! One method, three patterns - just like Python:
//! - Delimiter: `splitby(",")` → split by comma
//! - Whitespace: `splitby("")` → split on any whitespace
//! - Lines: `splitby("\n")` → split by newlines
//!
//! ## Examples
//!
//! ```rust
//! use webrust::prelude::*;
//! # fn example() {
//! // Smart splitting
//! let parts = "a,b,c".splitby(",");        // ["a", "b", "c"]
//! let words = "hello  world".splitby("");  // ["hello", "world"]
//! let lines = "L1\nL2\nL3".splitby("\n");  // ["L1", "L2", "L3"]
//!
//! // Fluent chaining
//! let result = "a,b,c".splitby(",").join(" → ");  // "a → b → c"
//!
//! // Case transformations
//! let upper = "hello".upper();             // "HELLO"
//! let title = "hello world".title();       // "Hello World"
//!
//! // Content validation
//! let is_alpha = "hello".isalpha();        // true
//! let starts = "user@example.com".startswith("user");  // true
//! # }
//! ```

// webrust/src/viz/string.rs
pub trait PyStr {
    fn lstrip(&self) -> &str;
    fn rstrip(&self) -> &str;
    fn ltrim(&self, pat: &str) -> &str;
    fn rtrim(&self, pat: &str) -> &str;
    fn removeprefix(&self, pre: &str) -> &str;
    fn removesuffix(&self, suf: &str) -> &str;
    fn startswith(&self, pat: &str) -> bool;
    fn endswith(&self, pat: &str) -> bool;
    fn lower(&self) -> String;
    fn upper(&self) -> String;
    fn title(&self) -> String;
    fn capitalize(&self) -> String;
    fn swapcase(&self) -> String;
    fn zfill(&self, width: usize) -> String;
    fn ljust(&self, width: usize, fill: char) -> String;
    fn rjust(&self, width: usize, fill: char) -> String;
    fn center(&self, width: usize, fill: char) -> String;
    fn isalpha(&self) -> bool;
    fn isdigit(&self) -> bool;
    fn isalnum(&self) -> bool;
    fn isupper(&self) -> bool;
    fn islower(&self) -> bool;
    fn splitby(&self, pattern: &str) -> Vec<&str>;
    fn count(&self, pattern: &str) -> usize;
}

impl PyStr for str {
    #[inline] fn lstrip(&self) -> &str { self.trim_start() }
    #[inline] fn rstrip(&self) -> &str { self.trim_end() }
    #[inline] fn ltrim(&self, pat: &str) -> &str { self.trim_start_matches(pat) }
    #[inline] fn rtrim(&self, pat: &str) -> &str { self.trim_end_matches(pat) }
    #[inline] fn removeprefix(&self, pre: &str) -> &str { self.strip_prefix(pre).unwrap_or(self) }
    #[inline] fn removesuffix(&self, suf: &str) -> &str { self.strip_suffix(suf).unwrap_or(self) }
    #[inline] fn startswith(&self, pat: &str) -> bool { self.starts_with(pat) }
    #[inline] fn endswith(&self, pat: &str) -> bool { self.ends_with(pat) }
    #[inline] fn lower(&self) -> String { self.to_lowercase() }
    #[inline] fn upper(&self) -> String { self.to_uppercase() }

    fn title(&self) -> String {
        let mut out = String::with_capacity(self.len());
        let mut new_word = true;
        for ch in self.chars() {
            if ch.is_whitespace() { new_word = true; out.push(ch); }
            else if new_word { for uc in ch.to_uppercase() { out.push(uc); } new_word = false; }
            else { for lc in ch.to_lowercase() { out.push(lc); } }
        }
        out
    }

    fn capitalize(&self) -> String {
        let mut it = self.chars();
        let mut out = String::with_capacity(self.len());
        if let Some(f) = it.next() { for uc in f.to_uppercase() { out.push(uc); } }
        for ch in it { for lc in ch.to_lowercase() { out.push(lc); } }
        out
    }

    fn swapcase(&self) -> String {
        let mut out = String::with_capacity(self.len());
        for ch in self.chars() {
            if ch.is_uppercase() { for lc in ch.to_lowercase() { out.push(lc); } }
            else if ch.is_lowercase() { for uc in ch.to_uppercase() { out.push(uc); } }
            else { out.push(ch); }
        }
        out
    }

    fn zfill(&self, width: usize) -> String {
        let (sign, body) = if self.starts_with('+') || self.starts_with('-') {
            self.split_at(1)
        } else { ("", self) };
        let need = width.saturating_sub(sign.len() + body.chars().count());
        let mut out = String::with_capacity(sign.len() + need + self.len());
        out.push_str(sign);
        out.extend(std::iter::repeat('0').take(need));
        out.push_str(body);
        out
    }

    fn ljust(&self, width: usize, fill: char) -> String {
        let len = self.chars().count();
        if len >= width { return self.to_string(); }
        let pad = width - len;
        let mut out = String::with_capacity(self.len() + pad * fill.len_utf8());
        out.push_str(self);
        out.extend(std::iter::repeat(fill).take(pad));
        out
    }

    fn rjust(&self, width: usize, fill: char) -> String {
        let len = self.chars().count();
        if len >= width { return self.to_string(); }
        let pad = width - len;
        let mut out = String::with_capacity(self.len() + pad * fill.len_utf8());
        out.extend(std::iter::repeat(fill).take(pad));
        out.push_str(self);
        out
    }

    fn center(&self, width: usize, fill: char) -> String {
        let len = self.chars().count();
        if len >= width { return self.to_string(); }
        let total = width - len;
        let left = total / 2;
        let right = total - left;
        let mut out = String::with_capacity(self.len() + total * fill.len_utf8());
        out.extend(std::iter::repeat(fill).take(left));
        out.push_str(self);
        out.extend(std::iter::repeat(fill).take(right));
        out
    }

    fn isalpha(&self) -> bool { !self.is_empty() && self.chars().all(|c| c.is_alphabetic()) }
    fn isdigit(&self) -> bool { !self.is_empty() && self.chars().all(|c| c.is_ascii_digit()) }
    fn isalnum(&self) -> bool { !self.is_empty() && self.chars().all(|c| c.is_alphanumeric()) }

    fn isupper(&self) -> bool {
        let mut cased = false;
        for ch in self.chars() {
            if ch.is_lowercase() { return false; }
            else if ch.is_uppercase() { cased = true; }
        }
        cased
    }

    fn islower(&self) -> bool {
        let mut cased = false;
        for ch in self.chars() {
            if ch.is_uppercase() { return false; }
            else if ch.is_lowercase() { cased = true; }
        }
        cased
    }

    #[inline]
    fn splitby(&self, pattern: &str) -> Vec<&str> {
        match pattern {
            "" => self.split_whitespace().collect(),
            "\n" => self.lines().collect(),
            pat => self.split(pat).collect(),
        }
    }

    #[inline]
    fn count(&self, pattern: &str) -> usize {
        self.matches(pattern).count()
    }
}

impl PyStr for &str {
    #[inline] fn lstrip(&self) -> &str { (*self).lstrip() }
    #[inline] fn rstrip(&self) -> &str { (*self).rstrip() }
    #[inline] fn ltrim(&self, pat: &str) -> &str { (*self).ltrim(pat) }
    #[inline] fn rtrim(&self, pat: &str) -> &str { (*self).rtrim(pat) }
    #[inline] fn removeprefix(&self, pre: &str) -> &str { (*self).removeprefix(pre) }
    #[inline] fn removesuffix(&self, suf: &str) -> &str { (*self).removesuffix(suf) }
    #[inline] fn startswith(&self, pat: &str) -> bool { (*self).startswith(pat) }
    #[inline] fn endswith(&self, pat: &str) -> bool { (*self).endswith(pat) }
    #[inline] fn lower(&self) -> String { (*self).lower() }
    #[inline] fn upper(&self) -> String { (*self).upper() }
    #[inline] fn title(&self) -> String { (*self).title() }
    #[inline] fn capitalize(&self) -> String { (*self).capitalize() }
    #[inline] fn swapcase(&self) -> String { (*self).swapcase() }
    #[inline] fn zfill(&self, width: usize) -> String { (*self).zfill(width) }
    #[inline] fn ljust(&self, width: usize, fill: char) -> String { (*self).ljust(width, fill) }
    #[inline] fn rjust(&self, width: usize, fill: char) -> String { (*self).rjust(width, fill) }
    #[inline] fn center(&self, width: usize, fill: char) -> String { (*self).center(width, fill) }
    #[inline] fn isalpha(&self) -> bool { (*self).isalpha() }
    #[inline] fn isdigit(&self) -> bool { (*self).isdigit() }
    #[inline] fn isalnum(&self) -> bool { (*self).isalnum() }
    #[inline] fn isupper(&self) -> bool { (*self).isupper() }
    #[inline] fn islower(&self) -> bool { (*self).islower() }
    #[inline] fn splitby(&self, pattern: &str) -> Vec<&str> { (*self).splitby(pattern) }
    #[inline] fn count(&self, pattern: &str) -> usize { (*self).count(pattern) }
}

pub trait PyString { fn set<R: core::ops::RangeBounds<usize>>(&mut self, range: R, repl: &str); }
impl PyString for String { fn set<R: core::ops::RangeBounds<usize>>(&mut self, range: R, repl: &str) { self.replace_range(range, repl) } }

pub trait PyVec {
    fn join(&self, sep: &str) -> String;
}

impl PyVec for Vec<&str> {
    #[inline]
    fn join(&self, sep: &str) -> String {
        self.as_slice().join(sep)
    }
}

impl<const N: usize> PyVec for [&str; N] {
    #[inline]
    fn join(&self, sep: &str) -> String {
        self.as_slice().join(sep)
    }
}

impl PyVec for &[&str] {
    #[inline]
    fn join(&self, sep: &str) -> String {
        (*self).join(sep)
    }
}