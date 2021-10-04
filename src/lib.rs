#![allow(unused)]

#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect::<std::collections::HashMap<_, _>>(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
}

#[macro_export]
macro_rules! btree_map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect::<std::collections::BTreeMap<_, _>>(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
}

#[macro_export]
macro_rules! set {
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect::<std::collections::HashSet<_>>(IntoIterator::into_iter([$($v,)*]))
    }};
}

#[macro_export]
macro_rules! btree_set {
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect::<std::collections::BTreeSet<_>>(IntoIterator::into_iter([$($v,)*]))
    }};
}
