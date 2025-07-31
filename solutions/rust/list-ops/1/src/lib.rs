/// Yields each item of a and then each item of b
pub fn append<T, I>(_a: I, _b: I) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>+ std::fmt::Debug,T: std::fmt::Debug
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    let mut v = vec![];
    for i in _a {
        v.push(i);
    }
    for i in _b {
        v.push(i);
    }
    v.into_iter()
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I, NI, T>(_nested_iter: I) -> impl Iterator<Item = T>
where
    NI: Iterator<Item = T> + std::fmt::Debug,
    I: Iterator<Item = NI>,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // _nested_iter.flatten()
    let mut v = vec![];    
    for i in _nested_iter {
        for j in i {
            v.push(j);
        }
    }
    v.into_iter()
    // std::iter::from_fn(|| todo!())
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, T, F>(_iter: I, _predicate: F) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
    T: std::fmt::Debug
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    let mut v = vec![];

    for i in _iter {
        if _predicate(&i) {
            v.push(i);
        }
    }
    v.into_iter()
}

pub fn length<I: Iterator<Item = T>, T>(_iter: I) -> usize {
    let mut count = 0;
    for _ in _iter {
        count += 1;
    }
    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, T, U>(_iter: I, _function: F) -> impl Iterator<Item = U>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    let mut v = vec![];

    for i in _iter {
        v.push(_function(i));
    }
    v.into_iter()
}

pub fn foldl<I, F, T, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: Iterator<Item = T>,
    F: Fn(U, T) -> U,
{
    let mut acc = _initial;
    for v in _iter {
        acc = _function(acc, v);
    }
    acc
}

pub fn foldr<I, F, T, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: DoubleEndedIterator<Item = T>,
    F: Fn(U, T) -> U,
{
 let _iter = reverse(_iter);
    let mut acc = _initial;
    for v in _iter {
        acc = _function(acc, v);
    }
    acc
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator<Item = T>,T>(_iter: I) -> impl Iterator<Item = T> {
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    
    let mut _iter = _iter;
    let mut v = vec![];
    while let Some(val) = _iter.next_back() {
        v.push(val);
    }
    v.into_iter()
}
                