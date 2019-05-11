use map_retry::MapRetry;

static mut EVEN: bool = true;

fn even_fail<T>(a: T) -> Option<T> {
    unsafe {
        EVEN = !EVEN;
    }
    if unsafe { EVEN } {
        return None;
    }
    Some(a)
}

#[test]
fn should_return_all_elements() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let res: Vec<_> = a.iter().map_retry(|a| even_fail(a).ok_or(())).collect();
    assert_eq!(a.len(), res.len());
}

#[test]
fn should_retry_on_error() {
    let mut even = true;

    let even_fail = |a| {
        even = !even;
        if even {
            return Err(());
        }
        Ok(a)
    };

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut it = a.iter().map_retry(even_fail);
    assert_eq!(it.next().unwrap(), Ok(&1));
    assert_eq!(it.next().unwrap(), Ok(&3));
    assert_eq!(it.next().unwrap(), Ok(&5));
    assert_eq!(it.next().unwrap(), Ok(&7));
    assert_eq!(it.next().unwrap(), Ok(&9));
    assert_eq!(it.next().unwrap(), Ok(&10));
    // Finished normal run, retrying failed
    assert_eq!(it.next().unwrap(), Err(()));
    assert_eq!(it.next().unwrap(), Ok(&6));
    assert_eq!(it.next().unwrap(), Err(()));
    assert_eq!(it.next().unwrap(), Ok(&2));
    // Half of the failed succeded
    assert_eq!(it.next(), None);
}

#[test]
fn should_work_with_iterators() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let res: Vec<_> = a
        .iter()
        .map_retry(|a| even_fail(a).ok_or(()))
        .filter(Result::is_ok)
        .collect();
    assert!(a.len() > res.len());
}
