pub fn vec_deque() {
    use std::collections::VecDeque;
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));

    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);

    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5));

    let mut d = VecDeque::new();
    d.push_back(1);
    d.push_back(2);

    assert_eq!(d.pop_front(), Some(1));
    assert_eq!(d.pop_front(), Some(2));
    assert_eq!(d.pop_front(), None);

    buf = VecDeque::new();
    assert_eq!(buf.pop_back(), None);
    buf.push_back(1);
    buf.push_back(3);
    assert_eq!(buf.pop_back(), Some(3));
}

fn main() {
    vec_deque();
    println!("Hello, world!");
}
