# Map retry

A zero dependency trait that provides `map_retry` function on top of native iterators.

Map retry crate provides a trait that allows to repeat mapping on failed results.
This is useful for doing IO such as loading webpages using iterators.

```
use map_retry::MapRetry;
fn retry() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let res: Vec<_> = a.iter().map_retry(|a| do_failable_io(a)).collect();
    assert_eq!(a.len(), res.len());
}
```