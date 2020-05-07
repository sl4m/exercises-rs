use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct SimpleLinkedList<T> {
    head: Link<T>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        self.len += 1;
        let new_head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
        self.head = new_head;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.len -= 1;
            self.head = head.next;
            head.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.data)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut new_list = Self::new();
        for elem in self.into_iter() {
            new_list.push(elem);
        }
        new_list
    }
}

pub struct IntoIter<T>(SimpleLinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for elem in iter.into_iter() {
            list.push(elem);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len);
        for elem in self.rev().into_iter() {
            vec.push(elem);
        }
        vec
    }
}
