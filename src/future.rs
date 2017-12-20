//! 'easytiming::future::Timing' provides mean to measure time taken by future execution
//! It is enabled by feature 'futures'
//!
//! Quick start
//!
//! ```rust
//! extern crate futures;
//! extern crate easytiming;
//!
//! use easytiming::future::FutureExt;
//! use easytiming::future::Timing;
//! use futures::future::ok;
//!
//! fn main() {
//!     let ok = ok::<u8, u8>(1);
//!     let future = ok.timing("ok future");
//!
//!     // Do some important stuff here
//!     // ...
//! }
//! ```

use std::fmt;
use std::io::{Stdout, Write};
use std::time::{Duration, Instant};
use std::borrow::Cow;

use futures::{Future, Async, Poll};

use super::Sink;

#[derive(Debug)]
pub struct Timing<'a, A, W = Stdout>
where
    A: Future,
    W: Write,
{
    inner: A,
    start: Instant,
    completed: Option<Instant>,
    lapse: Duration,
    name: Cow<'a, str>,
    quiet: bool,
    sink: Sink<W>,
}

impl<'a, A, W> fmt::Display for Timing<'a, A, W>
where
    A: Future,
    W: Write,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Timing({}) is running for {:?}",
            self.name(),
            self.elapsed()
        )
    }
}

impl<'a, A, W> Timing<'a, A, W>
where
    A: Future,
    W: Write,
{
    pub fn new<N>(inner: A, name: N) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Self {
            inner,
            start: Instant::now(),
            completed: None,
            lapse: Duration::default(),
            name: name.into(),
            quiet: false,
            sink: Sink::Println,
        }
    }

    pub fn with_writer<N>(inner: A, name: N, writer: W) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        Self {
            inner,
            start: Instant::now(),
            completed: None,
            lapse: Duration::default(),
            name: name.into(),
            quiet: false,
            sink: Sink::Writer(writer),
        }
    }

    #[cfg(log)]
    pub fn with_writer<N>(name: N, writer: W) -> Self
    where
        N: Into<Cow<'a, str>>,
    {
        let mut timing = Self::default();
        timing.name = name.into();
        timing.writer = Some(writer);
        timing
    }

    #[inline]
    fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }

    #[inline]
    fn finish(&mut self) {
        self.lapse = self.elapsed();
        if self.quiet {
            return;
        }
        self.report()
    }

    #[inline]
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn report(&mut self) {
        let output = format!(
            "\"{}\" was running for {} ns",
            self.name,
            self.lapse.subsec_nanos()
        );
        match self.sink {
            Sink::Println => println!("{}", output),
            Sink::Writer(ref mut out) => write!(out, "{}", output).unwrap(),
            #[cfg(log)]
            Sink::Log => trace!(output),
            #[cfg(slog)]
            Sink::Slog => trace!(output),
        }
    }
}

impl<'a, A, W> Drop for Timing<'a, A, W>
where
    A: Future,
    W: Write,
{
    fn drop(&mut self) {
        self.finish()
    }
}


impl<'a, A, W> Future for Timing<'a, A, W>
where
    A: Future,
    W: Write
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let poll = self.inner.poll();
        match poll {
            Ok(Async::Ready(_)) | Err(_) => self.completed = Some(Instant::now()),
            _ => (),
        }
        poll
    }
}

pub trait FutureExt: Future {
    fn timing<'a, N>(self, name: N) -> Timing<'a, Self>
    where
        N: Into<Cow<'a, str>>,
        Self: Sized;
}

impl<F> FutureExt for F
where
    F: Future
{
    fn timing<'a, N>(self, name: N) -> Timing<'a, Self>
    where
        N: Into<Cow<'a, str>>,
    {
        Timing::new(self, name)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::ok;

    const NAME: &str = "timing";

    #[test]
    fn fromok() {
        let ok = ok::<u64, u64>(1);
        let t = ok.timing(NAME);
        assert_eq!(t.name(), NAME);
    }
}
