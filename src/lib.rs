//! easytiming provides tools for taking timing measurement for functions, code blocks
//! and other elements of Rust source code. It is flexible enough to accomodate different
//! output options. It plays nice with `log` and `slog`. It works on stable in its basic form.
//! In addition, when used on nightly, it can be invoked as an attribute.

use std::fmt;
use std::time;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Timing<'a> {
    start: time::Instant,
    name: Cow<'a, str>,
}

impl<'a> fmt::Display for Timing<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Timing({}) is running for {:?}",
            self.name(),
            self.elapsed()
        )
    }
}

impl<'a> Timing<'a> {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Self {
            start: time::Instant::now(),
            name: name.into(),
        }
    }

    #[inline]
    fn elapsed(&self) -> time::Duration {
        time::Instant::now() - self.start
    }

    #[inline]
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: &str = "timing";

    #[test]
    fn fromstr() {
        let t = Timing::new(NAME);
        assert_eq!(t.name(), NAME);
    }

    #[test]
    fn fromstring() {
        let t = Timing::new(String::from(NAME));
        assert_eq!(t.name(), NAME);
    }

    #[test]
    fn fromborrowed() {
        let t = Timing::new(Cow::Borrowed(NAME));
        assert_eq!(t.name(), NAME);
    }

    #[test]
    fn fromowned() {
        let t = Timing::new(Cow::Owned(String::from(NAME)));
        assert_eq!(t.name(), NAME);
    }
}
