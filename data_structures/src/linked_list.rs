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

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn pop_first(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                self.length -= 1;
                Some(node.element)
            }
            None => None,
        }
    }

    pub fn add_first(&mut self, arg: T) {
        let new_node = Box::new(Node::new(arg, self.head.take()));
        self.head = Some(new_node);
        self.length += 1;
    }

    #[must_use]
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    #[must_use]
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }

    fn clear(&mut self) {
        let mut current = self.head.take();
        while let Some(mut boxed_node) = current {
            current = boxed_node.next.take();
        }
        self.length = 0;
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(vector: Vec<T>) -> Self {
        let mut list = Self::new();
        for item in vector.into_iter().rev() {
            list.add_first(item);
        }
        list
    }
}

impl<T: Clone> From<&[T]> for LinkedList<T> {
    fn from(slice: &[T]) -> Self {
        let mut list = Self::new();
        for item in slice.iter().rev() {
            list.add_first(item.clone());
        }
        list
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

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_first()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_an_empty_list() {
        let list: LinkedList<i32> = LinkedList::default();

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn creates_list_from_a_vector() {
        let vector = vec![1, 2, 3];

        let list: LinkedList<i32> = vector.clone().into();

        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        assert_eq!(Vec::from(list), vector);
    }

    #[test]
    fn creates_list_from_a_slice() {
        let slice = [2, 3, 4];

        let list: LinkedList<i32> = (&slice[..]).into();

        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        assert_eq!(Vec::from(list).as_slice(), slice);
    }

    #[test]
    fn adds_first_adds_an_element_at_the_start() {
        let mut list: LinkedList<i32> = LinkedList::new();
        let value = 23;

        list.add_first(value);

        assert_eq!(list.len(), 1);
        assert_eq!(list.head.as_ref().unwrap().element, value);
    }

    #[test]
    fn adds_first_adds_an_element_at_the_start_moving_the_head_element_forward() {
        let existing_element = 22;
        let mut list: LinkedList<i32> = vec![existing_element].into();
        let value = 23;

        list.add_first(value);

        assert_eq!(list.len(), 2);
        assert_eq!(list.head.as_ref().unwrap().element, value);
        assert_eq!(
            list.head.as_ref().unwrap().next.as_ref().unwrap().element,
            existing_element
        );
    }

    #[test]
    fn peeks_front_item_without_removing_it_or_consuming_ownership() {
        let list: LinkedList<i32> = vec![2, 3, 4].into();

        let front_element: Option<&i32> = list.peek_front();

        assert_eq!(front_element.unwrap(), &2);
        assert_eq!(Vec::from(list), vec![2, 3, 4]);
    }

    #[test]
    fn peeks_front_item_mutably_and_allows_in_place_modification() {
        let mut list: LinkedList<i32> = vec![10, 20, 30].into();

        if let Some(front_element) = list.peek_front_mut() {
            *front_element = 42;
        }

        assert_eq!(list.peek_front(), Some(&42));
        assert_eq!(Vec::from(list), vec![42, 20, 30]);
    }

    #[test]
    fn clears_all_elements_from_the_list() {
        let mut list: LinkedList<i32> = vec![1, 2, 3, 4, 5].into();

        list.clear();

        assert!(list.is_empty());
    }

    #[test]
    fn into_iter_consumes_list_and_yields_owned_elements() {
        let list: LinkedList<i32> = vec![1, 2, 3].into();

        let mut values = Vec::new();
        for item in list {
            values.push(item);
        }

        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn iter_borrows_list_and_allows_multiple_passes() {
        let list: LinkedList<i32> = vec![10, 20, 30].into();

        let first_pass: Vec<&i32> = list.iter().collect();
        let second_pass: Vec<&i32> = (&list).into_iter().collect();

        assert_eq!(first_pass, vec![&10, &20, &30]);
        assert_eq!(second_pass, vec![&10, &20, &30]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn iter_mut_allows_in_place_element_modification() {
        let mut list: LinkedList<i32> = vec![1, 2, 3].into();

        for val in &mut list {
            *val *= 10;
        }

        let result: Vec<i32> = Vec::from(list);
        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    fn iterators_support_standard_combinators() {
        let list: LinkedList<i32> = vec![1, 2, 3, 4, 5].into();

        let sum: i32 = list.iter().filter(|&&x| x % 2 != 0).sum();

        assert_eq!(sum, 9);
    }
}
