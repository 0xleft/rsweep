// tests for ip range iterator
#[path = "../src/sweeper/range.rs"]
mod range;

use range::IpRangeIterator;
use std::net::Ipv4Addr;

#[test]
fn test_ip_range_iterator() {
    let start = Ipv4Addr::new(127, 0, 0, 1);
    let end = Ipv4Addr::new(127, 0, 0, 3);
    let mut iter = IpRangeIterator::new(start, end);

    assert_eq!(iter.next(), Some(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(iter.next(), Some(Ipv4Addr::new(127, 0, 0, 2)));
    assert_eq!(iter.next(), Some(Ipv4Addr::new(127, 0, 0, 3)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_ip_step() {
    let start = Ipv4Addr::new(127, 0, 255, 254);
    let end = Ipv4Addr::new(127, 1, 0, 1);
    let mut iter = IpRangeIterator::new(start, end);

    assert_eq!(iter.next(), Some(Ipv4Addr::new(127, 0, 255, 254)));
    assert_eq!(iter.next(), Some(Ipv4Addr::new(127, 0, 255, 255)));
    assert_eq!(iter.next(), Some(Ipv4Addr::new(127, 1, 0, 0)));
    assert_eq!(iter.next(), Some(Ipv4Addr::new(127, 1, 0, 1)));
    assert_eq!(iter.next(), None);
}