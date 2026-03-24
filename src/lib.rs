use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::rc::Weak;
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { 
            value, 
            next: None, 
            prev: None 
        }))
    }
}

impl<T> DoublyLinkedList<T> {

    pub fn new() -> Self {
        DoublyLinkedList { 
            head: None, 
            tail: None, 
            len: 0 
        }
    }

    pub fn push_front(&mut self, value: T){  //добавление элемента в начало
        let new_node = Node::new(value);

        match self.head.take() {
            Some(ex_head) => {
                ex_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(ex_head);

                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
        }

        self.len += 1;
        
    } 

    fn push_back(&mut self, value: T) { //добавление элемента в конец 
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(ex_tail) => {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&ex_tail));
                ex_tail.borrow_mut().next = Some(Rc::clone(&new_node));

                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }
    }

    fn pop_front() {} //удаление элемента с начала

    fn pop_back() {} //удаление элемента с конца 

    fn add_index() {} //вставка элемента по индексу

    fn delete_index() {} //удаление элемента по индексу

    fn search_value() {} //поиск элемента по значению
}








#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(10);
        list.push_front(20);

        assert_eq!(list.len, 2);
    }

    fn test_push_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(40);
        list.push_back(30);
        list.push_front(8);


        assert_eq!(list.len, 3);
    }

}
