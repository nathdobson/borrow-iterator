use itertools::EitherOrBoth;

use crate::{once, BorrowIterator, ConstF, IteratorExt, RefF, UnsizedRefF};

#[test]
fn test_once() {
    let mut it = once(1);
    assert_eq!(it.next(), Some(&mut 1));
    assert_eq!(it.next(), None);
}

#[test]
fn test_as_streaming_variant() {
    let vec = vec![0u8, 1, 2, 3];
    let mut it = vec.windows(2).as_streaming::<UnsizedRefF<[u8]>, _>(|x| x);
    assert_eq!(it.next(), Some(&[0u8, 1] as &[u8]));
    assert_eq!(it.next(), Some(&[1u8, 2] as &[u8]));
    assert_eq!(it.next(), Some(&[2u8, 3] as &[u8]));
    assert_eq!(it.next(), None);
}

#[test]
fn test_as_std() {
    let mut it = once(1).map::<ConstF<usize>, _>(|&(), x| x.clone()).as_std();
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), None);
}

#[test]
fn test_map_first() {
    let vec = vec![0u8, 1, 2, 3];
    let it = vec.windows(2).as_streaming::<UnsizedRefF<[u8]>, _>(|x| x);
    let mut it = it.map::<RefF<ConstF<u8>>, _>(|&(), x: &[u8]| x.first().unwrap());
    assert_eq!(it.next(), Some(&0));
}

#[test]
fn test_zip_longest() {
    let vec = vec![0u8, 1, 2, 3];
    let it1 = vec.windows(2).as_streaming::<UnsizedRefF<[u8]>, _>(|x| x);
    let it2 = vec.windows(3).as_streaming::<UnsizedRefF<[u8]>, _>(|x| x);
    let mut it3 = it1.zip_longest(it2);
    assert_eq!(
        it3.next(),
        Some(EitherOrBoth::Both(
            &[0u8, 1] as &[u8],
            &[0u8, 1, 2] as &[u8]
        ))
    );
    assert_eq!(
        it3.next(),
        Some(EitherOrBoth::Both(
            &[1u8, 2] as &[u8],
            &[1u8, 2, 3] as &[u8]
        ))
    );
    assert_eq!(it3.next(), Some(EitherOrBoth::Left(&[2u8, 3] as &[u8])));
    assert_eq!(it3.next(), None);
}

#[test]
fn test_filter() {
    let vec = vec![0u8, 1, 2, 3];
    let it = vec.windows(2).as_streaming::<UnsizedRefF<[u8]>, _>(|x| x);
    let mut it = it.filter(|x| x[0] % 2 == 0);
    assert_eq!(it.next(), Some(&[0u8, 1] as &[u8]));
    assert_eq!(it.next(), Some(&[2u8, 3] as &[u8]));
    assert_eq!(it.next(), None);
}

#[test]
fn test_collect() {
    let vec = vec![0u8, 1, 2, 3];
    let it = vec.windows(2).as_streaming::<UnsizedRefF<[u8]>, _>(|x| x);
    it.cloned().collect::<Vec<u8>, Vec<Vec<u8>>>();
}
