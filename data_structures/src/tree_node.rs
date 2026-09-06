use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node<T> {
    pub value: T,
    pub parent: RefCell<Weak<Node<T>>>,
    pub children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    #[must_use]
    fn new(value: T) -> Rc<Node<T>> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    fn add_child(self: &Rc<Self>, child: Rc<Node<T>>) {
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

        assert_eq!(root.value, 1);

        let children = root.children.borrow();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].value, 2);
    }
}
