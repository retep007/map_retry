//! Map retry crate provides a trait that allows to repeat mapping on failed results.
//! This is useful for doing I/O such as loading webpages using iterators.
//! 
//! `map_retry` behaves like normal map function, with exception that return type
//! must be `Result` and if `Err` is returned it tries to execute mapping function
//! one more time after all original items have been processed. **Order** of results is
//! not guaranteed. If mapping fails also on second try the last error is returned.
//! The same number of input and output items is guaranteed.
//! 
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

pub mod options;
pub mod delayed;

use options::Options;
use delayed::DelayedRetryIter;

/// Trait defining retry signatures
pub trait MapRetry: Iterator + Sized {
    /// Works the same as map function, but retries failures.
    /// Return type of provided closure must of type `Result` if result is error
    /// iterator retries to apply function agian.
    ///
    /// **Order** of elements is not guaranteed.
    /// All elements in original iterator are returned.
    fn map_retry<F>(self, f: F) -> DelayedRetryIter<Self, F>;

    fn map_retry_options<F>(self, options: Options, f: F) -> DelayedRetryIter<Self, F>;
}

impl<T: Iterator> MapRetry for T {
    /// Runs map function which retries results that return error.
    ///
    /// Errors are retried only after all elements have been mapped.
    /// Maping function must return `Result` type.
    /// Items are cloned when error is returned.
    fn map_retry<F>(self, f: F) -> DelayedRetryIter<Self, F>  {
        DelayedRetryIter {
            iter: self,
            options: Default::default(),
            failed: Default::default(),
            f,
        }
    }

    fn map_retry_options<F>(self, options: Options, f: F) -> DelayedRetryIter<Self, F> {
        DelayedRetryIter {
            iter: self,
            options,
            failed: Default::default(),
            f,
        }
    }
}