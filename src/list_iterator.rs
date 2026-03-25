use std::rc::Rc;
use std::cell::RefCell;
use crate::Node;

pub struct ListIterator<T> {
    current: Option<Rc<RefCell<Node<T>>>>
}

impl<T: Clone> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            Some(node) => {
                let ref_node = node.borrow();
                self.current = ref_node.next.clone();
                Some(ref_node.value.clone())
            },
            None => None,
        }
    }
}