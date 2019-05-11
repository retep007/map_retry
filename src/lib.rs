//! Map retry crate provides a trait that allows to repeat mapping on failed results.
//! This is useful for doing IO such as loading webpages using iterators.
//! ```
//! use map_retry::MapRetry;
//! # static mut EVEN: bool = true;
//! #
//! # fn do_failable_io<T>(a: T) -> Result<T, ()> {
//! #     unsafe {
//! #         EVEN = !EVEN;
//! #     }
//! #     if unsafe { EVEN } {
//! #         return Err(());
//! #     }
//! #     Ok(a)
//! # }
//! fn retry() {
//!     let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//! # // Uses closure on purpose
//!     let res: Vec<_> = a.iter().map_retry(|a| do_failable_io(a)).collect();
//!     assert_eq!(a.len(), res.len());
//! }
//! ```

/// Return type used for chaining with iterators
#[derive(Debug, Clone)]
pub struct MapIter<Iter: Iterator, F> {
    iter: Iter,
    failed: Vec<Iter::Item>,
    f: F,
}

impl<Iter: Iterator, F> MapIter<Iter, F> {
    fn new(iter: Iter, f: F) -> Self {
        MapIter {
            iter,
            failed: vec![],
            f,
        }
    }
}

/// Trait defining retry signatures
pub trait MapRetry: Iterator + Sized {
    /// Works the same as map function, but retries failures.
    /// Return type of provided closure must of type `Result` if result is error
    /// iterator retries to apply function agian.
    ///
    /// **Order** of elements is not guaranteed.
    /// All elements in original iterator are returned.
    fn map_retry<F>(self, f: F) -> MapIter<Self, F>;
    // fn map_retry<F>(self, f: F) -> MapIter<Self, F> where MapIter<Self, F>: Iterator;
}

impl<T: Iterator> MapRetry for T {
    /// Runs map function which retries results that return error.
    ///
    /// Errors are retried only after all elements have been mapped.
    /// Maping function must return `Result` type.
    /// Items are cloned when error is returned.
    fn map_retry<F>(self, f: F) -> MapIter<Self, F> {
        MapIter::new(self, f)
    }
}

impl<Iter: Iterator, F: FnMut(Iter::Item) -> Result<Out, E>, Out, E> Iterator for MapIter<Iter, F>
where
    Iter::Item: Clone,
{
    type Item = Result<Out, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res;
        while {
            res = self.iter.next();
            res.is_some()
        } {
            let res = res?;
            let m = (self.f)(res.clone());
            if m.is_ok() {
                return Some(m);
            } else {
                self.failed.push(res);
            }
        }
        Some((self.f)(self.failed.pop()?))
    }
}
