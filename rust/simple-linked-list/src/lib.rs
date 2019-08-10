use std::mem;

type NodePointer<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    next: NodePointer<T>,
}

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: NodePointer<T>,
    count: usize,
}

fn create<T>(element: T) -> NodePointer<T> {
    Some(Box::new(Node {
        data: element,
        next: None,
    }))
}

fn data<T>(pointer: &NodePointer<T>) -> Option<&T> {
    pointer.as_ref().map(|node| &node.data)
}

fn next<T>(pointer: &NodePointer<T>) -> &NodePointer<T> {
    &pointer.as_ref().unwrap().next
}

fn next_mut<T>(pointer: &mut NodePointer<T>) -> &mut NodePointer<T> {
    &mut pointer.as_mut().unwrap().next
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn push(&mut self, element: T) {
        let mut current = &mut self.head;
        while current.is_some() {
            current = next_mut(current);
        }
        self.count += 1;
        mem::replace(current, create(element));
    }

    pub fn push_front(&mut self, element: T) {
        let old_head = mem::replace(&mut self.head, create(element));
        *next_mut(&mut self.head) = old_head;
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut current = &mut self.head;
        while current.is_some() && next(current).is_some() {
            current = next_mut(current);
        }
        self.count = self.count.checked_sub(1).unwrap_or(0);
        current.take().map(|node| (*node).data)
    }

    pub fn peek(&self) -> Option<&T> {
        data(&self.head)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list: SimpleLinkedList<T> = SimpleLinkedList::new();

        let mut current = &self.head;
        while current.is_some() {
            list.push_front(data(current).unwrap().clone());
            current = next(current);
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(items: &[T]) -> Self {
        let mut list: SimpleLinkedList<T> = SimpleLinkedList::new();
        for item in items {
            list.push(item.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(item) = self.pop() {
            vec.insert(0, item);
        }
        vec
    }
}
