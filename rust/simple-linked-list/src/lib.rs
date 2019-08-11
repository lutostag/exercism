type NodePointer<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    next: NodePointer<T>,
}

pub struct SimpleLinkedList<T> {
    head: NodePointer<T>,
    count: usize,
}

fn data<T>(pointer: &NodePointer<T>) -> Option<&T> {
    pointer.as_ref().map(|node| &node.data)
}

fn next<T>(pointer: &NodePointer<T>) -> &NodePointer<T> {
    &pointer.as_ref().unwrap().next
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
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.count -= 1;
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
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
            list.push(data(current).unwrap().clone());
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
