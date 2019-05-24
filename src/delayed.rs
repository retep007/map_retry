use delay_queue::{Delay, DelayQueue};
use std::time::Duration;
use crate::options::Options;

/// Return type used for chaining with iterators
#[derive(Debug, Clone)]
pub struct DelayedRetryIter<Iter: Iterator, F> {
    pub (crate) iter: Iter,
    pub (crate) options: Options,
    pub (crate) failed: DelayQueue<Delay<Item<Iter::Item>>>,
    pub (crate) f: F,
}

#[derive(Debug, Clone)]
pub (crate) struct Item<T> {
    retries: u8,
    value: T,
}

impl<Iter: Iterator, F> DelayedRetryIter<Iter, F> {
    fn retry_failed(&mut self, res: Item<Iter::Item>) {
        if res.retries > 0 {
            let item = Item {
                retries: res.retries - 1,
                value: res.value
            };
            self.failed.push(Delay::for_duration(item, self.options.min_delay.clone().unwrap()))
        }
    }
}

impl<Iter: Iterator, F: FnMut(Iter::Item) -> Result<Out, E>, Out, E> Iterator for DelayedRetryIter<Iter, F>
where
    Iter::Item: Clone,
{
    type Item = Result<Out, E>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if something is available in failed queue
        let item = self.failed.try_pop_for(Duration::from_secs(0));
        // If have available failed element
        if let Some(res) = item {
            let item = (self.f)(res.value.value.clone());
            if item.is_ok() {
                return Some(item);
            } else {
                self.retry_failed(res.value);
            }
        }
        // Try to succeed with regular queue
        for item in self.iter.by_ref() {
            let res = (self.f)(item.clone());
            if res.is_ok() {
                return Some(res);
            } else {
                let item = Item {
                    retries: self.options.num_retries,
                    value: item
                };
                self.failed.push(Delay::for_duration(item, self.options.min_delay.clone().unwrap()))
            }
        }
        // No other items in regular queue
        // Block failed queue until something is available
        while !self.failed.is_empty() {
            let res = self.failed.pop();
            let item = (self.f)(res.value.value.clone());
            if item.is_ok() {
                return Some(item);
            } else {
                self.retry_failed(res.value);
            }
        }
        None
    }
}