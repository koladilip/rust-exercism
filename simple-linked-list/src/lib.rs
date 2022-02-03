use std::iter::FromIterator;

#[derive(PartialEq, Clone)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(PartialEq, Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }

    pub fn len(&self) -> usize {
        match &self.next {
            None => 1,
            Some(node) => 1 + node.len(),
        }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        match &self.head {
            None => 0,
            Some(node) => node.len(),
        }
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node::new(element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &(head.data))
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut current_head = self.head;
        let mut new_list = SimpleLinkedList::new();
        while let Some(node) = current_head {
            new_list.push(node.data);
            current_head = node.next;
        }
        return new_list;
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        _iter
            .into_iter()
            .fold(SimpleLinkedList::new(), |mut list, item| {
                list.push(item);
                list
            })
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

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut new_vec: Vec<T> = Vec::new();
        let mut current_head = _linked_list.head;
        while let Some(node) = current_head {
            new_vec.insert(0, node.data);
            current_head = node.next;
        }
        new_vec
    }
}
