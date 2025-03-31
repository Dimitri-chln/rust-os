use core::str::Split;

use alloc::{borrow::ToOwned, string::String};

pub struct PathBuf {
    inner: String,
}

impl PathBuf {
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

impl From<String> for PathBuf {
    fn from(value: String) -> Self {
        Self { inner: value }
    }
}

impl From<&str> for PathBuf {
    fn from(value: &str) -> Self {
        PathBuf::from(value.to_owned())
    }
}

pub struct Iter<'a> {
    split: Split<'a, char>,
}

impl<'a> Iter<'a> {
    fn new(value: &'a PathBuf) -> Self {
        Self {
            split: value.inner.as_str().split('/'),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.split.next() {
            Some("") => self.next(),
            Some(part) => Some(part),
            None => None,
        }
    }
}
