pub(crate) struct Node<T> {
    pub(crate) element: T,
    pub(crate) next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub(crate) fn new(element: T, next: Option<Box<Node<T>>>) -> Self {
        Self { element, next }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn from(vector: Vec<T>) -> Self {
        let mut list = Self::new();

        for item in vector.into_iter().rev() {
            list.add_first(item);
        }

        list
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn pop_first(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.element
        })
    }

    pub fn add_first(&mut self, arg: T) {
        let new_node = Box::new(Node::new(arg, self.head.take()));
        self.head = Some(new_node);
        self.length += 1;
    }
}

impl<T> From<LinkedList<T>> for Vec<T> {
    fn from(mut list: LinkedList<T>) -> Self {
        let mut vec = Vec::with_capacity(list.length);
        while let Some(element) = list.pop_first() {
            vec.push(element);
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();

        assert_eq!(list.length, 0);
        assert!(list.is_empty());
    }

    #[test]
    fn creates_list_from_a_vector() {
        let existing_elements = vec![2, 3, 4];

        let list: LinkedList<i32> = LinkedList::from(existing_elements.clone());

        assert_eq!(list.length, 3);
        assert!(!list.is_empty());
        assert_eq!(Vec::from(list), existing_elements);
    }

    #[test]
    fn adds_first_adds_an_element_at_the_start() {
        let mut list: LinkedList<i32> = LinkedList::new();
        let value = 23;

        list.add_first(value);

        assert_eq!(list.length, 1);
        assert_eq!(list.head.unwrap().element, value);
    }

    #[test]
    fn adds_first_adds_an_element_at_the_start_moving_the_head_element_forward() {
        let existing_element = 22;
        let mut list: LinkedList<i32> = LinkedList::from(vec![existing_element]);
        let value = 23;

        list.add_first(value);

        assert_eq!(list.length, 2);
        assert_eq!(list.head.as_ref().unwrap().element, value);
        assert_eq!(list.head.unwrap().next.unwrap().element, existing_element);
    }
}
