use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node<T> {
    pub value: RefCell<T>,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    #[must_use]
    pub fn new(value: T) -> Rc<Node<T>> {
        Rc::new(Node {
            value: RefCell::new(value),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    pub fn add_child(self: &Rc<Self>, child: Rc<Node<T>>) {
        *child.parent.borrow_mut() = Rc::downgrade(self);
        self.children.borrow_mut().push(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn creates_leaf_node_and_adds_child() {
        let root = Node::new(1);
        let leaf = Node::new(2);

        root.add_child(Rc::clone(&leaf));

        assert_eq!(*root.value.borrow(), 1);

        let children = root.children.borrow();
        assert_eq!(children.len(), 1);
        assert_eq!(*children[0].value.borrow(), 2);
    }

    #[test]
    fn mutates_node_value_using_interior_mutability() {
        let root = Node::new(10);
        let leaf = Node::new(20);
        root.add_child(Rc::clone(&leaf));

        *leaf.value.borrow_mut() += 5;

        let root_children = root.children.borrow();
        let child_value = *root_children[0].value.borrow();

        assert_eq!(child_value, 25);
    }

    #[test]
    fn child_can_look_up_to_parent_using_weak_smart_pointer() {
        let root = Node::new(10);
        let leaf = Node::new(20);
        root.add_child(Rc::clone(&leaf));

        let parent_as_weak_pointer = leaf.parent.borrow();
        let parent_as_rc_pointer = parent_as_weak_pointer.upgrade();

        assert!(parent_as_rc_pointer.is_some());

        let parent_node = parent_as_rc_pointer.unwrap();
        assert_eq!(*parent_node.value.borrow(), 10)
    }

    #[test]
    fn weak_pointer_becomes_none_when_parent_is_dropped() {
        let leaf = Node::new(200);

        {
            let root = Node::new(100);
            root.add_child(Rc::clone(&leaf));

            assert!(leaf.parent.borrow().upgrade().is_some());
            assert_eq!(Rc::strong_count(&root), 1);
        } // `root` goes out of scope here and is deallocated

        assert!(leaf.parent.borrow().upgrade().is_none());
    }
}
